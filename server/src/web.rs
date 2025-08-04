use std::{
    cmp::Ordering,
    convert::Infallible,
    sync::Arc,
    time::Duration,
    collections::HashMap,
};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use mongodb::{
    bson::doc,
    Client as MongoClient,
    Collection,
    IndexModel,
    options::{IndexOptions, UpdateOptions},
    results::UpdateResult,
};
use tokio::sync::{RwLock, Mutex};
use tokio_stream::StreamExt;
use warp::{
    Filter,
    filters::BoxedFilter,
    http::Uri,
    Reply,
};

use crate::{
    config::Config,
    ffxiv::Language,
    listing::PartyFinderListing,
    listing_container::{ListingContainer, QueriedListing},
    stats::CachedStatistics,
    template::listings::ListingsTemplate,
    template::stats::StatsTemplate,
};

mod stats;
pub mod api;

use crate::web::api::{ApiResponse, DetailedApiListing, ApiListing};

pub async fn start(config: Arc<Config>) -> Result<()> {
    let state = State::new(Arc::clone(&config)).await?;

    println!("listening at {}", config.web.host);
    warp::serve(router(state))
        .run(config.web.host)
        .await;
    Ok(())
}

pub struct State {
    mongo: MongoClient,
    stats: RwLock<Option<CachedStatistics>>,
    listings_cache: RwLock<ListingsCache>,
    detail_cache: RwLock<DetailCache>,
}

struct CacheEntry<T> {
    data: T,
    expires_at: DateTime<Utc>,
}

struct ListingsCache {
    entries: HashMap<String, CacheEntry<ApiResponse<Vec<ApiListing>>>>,
}

struct DetailCache {
    entries: HashMap<u32, CacheEntry<DetailedApiListing>>,
}

impl State {
    pub async fn new(config: Arc<Config>) -> Result<Arc<Self>> {
        let mongo = MongoClient::with_uri_str(&config.mongo.url)
            .await
            .context("could not create mongodb client")?;

        let state = Arc::new(Self {
            mongo,
            stats: Default::default(),
            listings_cache: RwLock::new(ListingsCache {
                entries: HashMap::new(),
            }),
            detail_cache: RwLock::new(DetailCache {
                entries: HashMap::new(),
            }),
        });

        state.collection()
            .create_index(
                IndexModel::builder()
                    .keys(mongodb::bson::doc! {
                        "listing.id": 1,
                        "listing.last_server_restart": 1,
                        "listing.created_world": 1,
                    })
                    .options(IndexOptions::builder()
                        .unique(true)
                        .build())
                    .build(),
                None,
            )
            .await
            .context("could not create unique index")?;

        state.collection()
            .create_index(
                IndexModel::builder()
                    .keys(mongodb::bson::doc! {
                        "updated_at": 1,
                    })
                    .build(),
                None,
            )
            .await
            .context("could not create updated_at index")?;

        state.collection()
            .create_index(
                IndexModel::builder()
                    .keys(mongodb::bson::doc! {
                        "listing.id": 1,
                    })
                    .build(),
                None,
            )
            .await
            .context("could not create listing.id index")?;
            
        state.collection()
            .create_index(
                IndexModel::builder()
                    .keys(mongodb::bson::doc! {
                        "listing.pf_category": 1,
                    })
                    .build(),
                None,
            )
            .await
            .context("could not create listing.pf_category index")?;
            
        state.collection()
            .create_index(
                IndexModel::builder()
                    .keys(mongodb::bson::doc! {
                        "listing.created_world": 1,
                    })
                    .build(),
                None,
            )
            .await
            .context("could not create listing.created_world index")?;
            
        state.collection()
            .create_index(
                IndexModel::builder()
                    .keys(mongodb::bson::doc! {
                        "listing.home_world": 1,
                    })
                    .build(),
                None,
            )
            .await
            .context("could not create listing.home_world index")?;
            
        state.collection()
            .create_index(
                IndexModel::builder()
                    .keys(mongodb::bson::doc! {
                        "listing.data_centre": 1,
                    })
                    .build(),
                None,
            )
            .await
            .context("could not create listing.data_centre index")?;
            
        state.collection()
            .create_index(
                IndexModel::builder()
                    .keys(mongodb::bson::doc! {
                        "listing.search_area": 1,
                    })
                    .build(),
                None,
            )
            .await
            .context("could not create listing.search_area index")?;

        state.collection()
            .create_index(
                IndexModel::builder()
                    .keys(mongodb::bson::doc! {
                        "listing.slots.job": 1,
                    })
                    .build(),
                None,
            )
            .await
            .context("could not create listing.slots.job index")?;
            
        state.collection()
            .create_index(
                IndexModel::builder()
                    .keys(mongodb::bson::doc! {
                        "listing.slots.accepting_classes": 1,
                    })
                    .build(),
                None,
            )
            .await
            .context("could not create listing.slots.accepting_classes index")?;

        let task_state = Arc::clone(&state);
        tokio::task::spawn(async move {
            loop {
                let all_time = match self::stats::get_stats(&*task_state).await {
                    Ok(stats) => stats,
                    Err(e) => {
                        eprintln!("error generating stats: {:#?}", e);
                        continue;
                    }
                };

                let seven_days = match self::stats::get_stats_seven_days(&*task_state).await {
                    Ok(stats) => stats,
                    Err(e) => {
                        eprintln!("error generating stats: {:#?}", e);
                        continue;
                    }
                };

                *task_state.stats.write().await = Some(CachedStatistics {
                    all_time,
                    seven_days,
                });

                tokio::time::sleep(Duration::from_secs(60 * 5)).await;
            }
        });

        let cache_state = Arc::clone(&state);
        tokio::task::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(60)).await;
                
                {
                    let mut cache = cache_state.listings_cache.write().await;
                    let now = Utc::now();
                    cache.entries.retain(|_, entry| entry.expires_at > now);
                }
                
                {
                    let mut cache = cache_state.detail_cache.write().await;
                    let now = Utc::now();
                    cache.entries.retain(|_, entry| entry.expires_at > now);
                }
            }
        });

        Ok(state)
    }

    pub fn collection(&self) -> Collection<ListingContainer> {
        self.mongo.database("rpf").collection("listings")
    }

    pub async fn get_listings_cache(&self, cache_key: &str) -> Option<ApiResponse<Vec<ApiListing>>> {
        let cache = self.listings_cache.read().await;
        if let Some(entry) = cache.entries.get(cache_key) {
            if entry.expires_at > Utc::now() {
                return Some(entry.data.clone());
            }
        }
        None
    }
    
    pub async fn set_listings_cache(&self, cache_key: String, data: ApiResponse<Vec<ApiListing>>, ttl_seconds: i64) {
        let mut cache = self.listings_cache.write().await;
        let expires_at = Utc::now() + chrono::Duration::seconds(ttl_seconds);
        cache.entries.insert(cache_key, CacheEntry { data, expires_at });
    }
    
    pub async fn get_detail_cache(&self, id: u32) -> Option<DetailedApiListing> {
        let cache = self.detail_cache.read().await;
        if let Some(entry) = cache.entries.get(&id) {
            if entry.expires_at > Utc::now() {
                return Some(entry.data.clone());
            }
        }
        None
    }
    
    pub async fn set_detail_cache(&self, id: u32, data: DetailedApiListing, ttl_seconds: i64) {
        let mut cache = self.detail_cache.write().await;
        let expires_at = Utc::now() + chrono::Duration::seconds(ttl_seconds);
        cache.entries.insert(id, CacheEntry { data, expires_at });
    }
}

fn router(state: Arc<State>) -> BoxedFilter<(impl Reply, )> {
    assets()
        .or(listings(Arc::clone(&state)))
        .or(stats(Arc::clone(&state)))
        .or(stats_seven_days(Arc::clone(&state)))
        .or(contribute(Arc::clone(&state)))
        .or(contribute_multiple(Arc::clone(&state)))
        .or(crate::web::api::listings_api(Arc::clone(&state)))
        .or(crate::web::api::listing_detail_api(Arc::clone(&state)))
        .or(index())
        .boxed()
}

fn assets() -> BoxedFilter<(impl Reply, )> {
    warp::get()
        .and(warp::path("assets"))
        .and(
            icons()
                .or(minireset())
                .or(common_css())
                .or(ffxiv_loadestone_ssf_ttf())
                .or(ffxiv_loadestone_ssf_woff())
                .or(listings_css())
                .or(listings_js())
                .or(stats_css())
                .or(stats_js())
                .or(d3())
                .or(pico())
                .or(common_js())
                .or(list_js())
        )
        .boxed()
}

fn icons() -> BoxedFilter<(impl Reply, )> {
    warp::path("icons.svg")
        .and(warp::path::end())
        .and(warp::fs::file("./assets/icons.svg"))
        .boxed()
}

fn minireset() -> BoxedFilter<(impl Reply, )> {
    warp::path("minireset.css")
        .and(warp::path::end())
        .and(warp::fs::file("./assets/minireset.css"))
        .boxed()
}

fn common_css() -> BoxedFilter<(impl Reply, )> {
    warp::path("common.css")
        .and(warp::path::end())
        .and(warp::fs::file("./assets/common.css"))
        .boxed()
}

fn ffxiv_loadestone_ssf_ttf() -> BoxedFilter<(impl Reply, )> {
    warp::path("FFXIV_Lodestone_SSF.ttf")
        .and(warp::path::end())
        .and(warp::fs::file("./assets/FFXIV_Lodestone_SSF.ttf"))
        .boxed()
}

fn ffxiv_loadestone_ssf_woff() -> BoxedFilter<(impl Reply, )> {
    warp::path("FFXIV_Lodestone_SSF.woff")
        .and(warp::path::end())
        .and(warp::fs::file("./assets/FFXIV_Lodestone_SSF.woff"))
        .boxed()
}

fn listings_css() -> BoxedFilter<(impl Reply, )> {
    warp::path("listings.css")
        .and(warp::path::end())
        .and(warp::fs::file("./assets/listings.css"))
        .boxed()
}

fn listings_js() -> BoxedFilter<(impl Reply, )> {
    warp::path("listings.js")
        .and(warp::path::end())
        .and(warp::fs::file("./assets/listings.js"))
        .boxed()
}

fn stats_css() -> BoxedFilter<(impl Reply, )> {
    warp::path("stats.css")
        .and(warp::path::end())
        .and(warp::fs::file("./assets/stats.css"))
        .boxed()
}

fn stats_js() -> BoxedFilter<(impl Reply, )> {
    warp::path("stats.js")
        .and(warp::path::end())
        .and(warp::fs::file("./assets/stats.js"))
        .boxed()
}

fn d3() -> BoxedFilter<(impl Reply, )> {
    warp::path("d3.js")
        .and(warp::path::end())
        .and(warp::fs::file("./assets/d3.v7.min.js"))
        .boxed()
}

fn pico() -> BoxedFilter<(impl Reply, )> {
    warp::path("pico.css")
        .and(warp::path::end())
        .and(warp::fs::file("./assets/pico.min.css"))
        .boxed()
}

fn common_js() -> BoxedFilter<(impl Reply, )> {
    warp::path("common.js")
        .and(warp::path::end())
        .and(warp::fs::file("./assets/common.js"))
        .boxed()
}

fn list_js() -> BoxedFilter<(impl Reply, )> {
    warp::path("list.js")
        .and(warp::path::end())
        .and(warp::fs::file("./assets/list.min.js"))
        .boxed()
}

fn index() -> BoxedFilter<(impl Reply, )> {
    let route = warp::path::end()
        .map(|| warp::redirect(Uri::from_static("/listings")));
    warp::get().and(route).boxed()
}

fn listings(state: Arc<State>) -> BoxedFilter<(impl Reply, )> {
    async fn logic(state: Arc<State>, codes: Option<String>) -> std::result::Result<impl Reply, Infallible> {
        let lang = Language::from_codes(codes.as_deref());

        let two_hours_ago = Utc::now() - chrono::Duration::hours(2);
        let res = state
            .collection()
            .aggregate(
                [
                    // don't ask me why, but mongo shits itself unless you provide a hard date
                    // doc! {
                    //     "$match": {
                    //         "created_at": {
                    //             "$gte": {
                    //                 "$dateSubtract": {
                    //                     "startDate": "$$NOW",
                    //                     "unit": "hour",
                    //                     "amount": 2,
                    //                 },
                    //             },
                    //         },
                    //     }
                    // },
                    doc! {
                        "$match": {
                            "updated_at": { "$gte": two_hours_ago },
                        }
                    },
                    doc! {
                        "$match": {
                            // filter private pfs
                            "listing.search_area": { "$bitsAllClear": 2 },
                        }
                    },
                    doc! {
                        "$set": {
                            "time_left": {
                                "$divide": [
                                    {
                                        "$subtract": [
                                            { "$multiply": ["$listing.seconds_remaining", 1000] },
                                            { "$subtract": ["$$NOW", "$updated_at"] },
                                        ]
                                    },
                                    1000,
                                ]
                            },
                            "minutes_since_update": {
                                "$divide": [
                                    { "$subtract": ["$$NOW", "$updated_at"] },
                                    60000
                                ]
                            },
                            "updated_minute": {
                                "$dateTrunc": {
                                    "date": "$updated_at",
                                    "unit": "minute",
                                    "binSize": 5,
                                },
                            },
                        }
                    },
                    doc! {
                        "$match": {
                            "$and": [
                                { "time_left": { "$gte": 0 } },
                                { "minutes_since_update": { "$lt": 5.0 } }
                            ]
                        }
                    },
                ],
                None,
            )
            .await;
        Ok(match res {
            Ok(mut cursor) => {
                let mut containers = Vec::new();

                while let Ok(Some(container)) = cursor.try_next().await {
                    let res: Result<QueriedListing> = try {
                        let result: QueriedListing = mongodb::bson::from_document(container)?;
                        result
                    };
                    if let Ok(listing) = res {
                        containers.push(listing);
                    }
                }

                containers.sort_by(|a, b| a.time_left.partial_cmp(&b.time_left).unwrap_or(Ordering::Equal));

                containers.sort_by_key(|container| container.listing.pf_category());
                containers.reverse();

                containers.sort_by_key(|container| container.updated_minute);
                containers.reverse();

                ListingsTemplate {
                    containers,
                    lang,
                }
            }
            Err(e) => {
                eprintln!("{:#?}", e);
                ListingsTemplate {
                    containers: Default::default(),
                    lang,
                }
            }
        })
    }

    let route = warp::path("listings")
        .and(warp::path::end())
        .and(
            warp::cookie::<String>("lang")
                .or(warp::header::<String>("accept-language"))
                .unify()
                .map(Some)
                .or(warp::any().map(|| None))
                .unify()
        )
        .and_then(move |codes: Option<String>| logic(Arc::clone(&state), codes));

    warp::get().and(route).boxed()
}

async fn stats_logic(state: Arc<State>, codes: Option<String>, seven_days: bool) -> std::result::Result<impl Reply, Infallible> {
    let lang = Language::from_codes(codes.as_deref());
    let stats = state.stats.read().await.clone();
    Ok(match stats {
        Some(stats) => StatsTemplate {
            stats: if seven_days { stats.seven_days } else { stats.all_time },
            lang,
        },
        None => panic!(),
    })
}

fn stats(state: Arc<State>) -> BoxedFilter<(impl Reply, )> {
    let route = warp::path("stats")
        .and(warp::path::end())
        .and(
            warp::cookie::<String>("lang")
                .or(warp::header::<String>("accept-language"))
                .unify()
                .map(Some)
                .or(warp::any().map(|| None))
                .unify()
        )
        .and_then(move |codes: Option<String>| stats_logic(Arc::clone(&state), codes, false));

    warp::get().and(route).boxed()
}

fn stats_seven_days(state: Arc<State>) -> BoxedFilter<(impl Reply, )> {
    let route = warp::path("stats")
        .and(warp::path("7days"))
        .and(warp::path::end())
        .and(
            warp::cookie::<String>("lang")
                .or(warp::header::<String>("accept-language"))
                .unify()
                .map(Some)
                .or(warp::any().map(|| None))
                .unify()
        )
        .and_then(move |codes: Option<String>| stats_logic(Arc::clone(&state), codes, true));

    warp::get().and(route).boxed()
}

fn contribute(state: Arc<State>) -> BoxedFilter<(impl Reply, )> {
    async fn logic(state: Arc<State>, listing: PartyFinderListing) -> std::result::Result<impl Reply, Infallible> {
        if listing.seconds_remaining > 60 * 60 {
            return Ok("invalid listing".to_string());
        }

        let result = insert_listing(&*state, listing).await;
        Ok(format!("{:#?}", result))
    }

    let route = warp::path("contribute")
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(move |listing: PartyFinderListing| logic(Arc::clone(&state), listing));
    warp::post().and(route).boxed()
}

fn contribute_multiple(state: Arc<State>) -> BoxedFilter<(impl Reply, )> {
    async fn logic(state: Arc<State>, listings: Vec<PartyFinderListing>) -> std::result::Result<impl Reply, Infallible> {
        let total = listings.len();
        let mut successful = 0;

        for listing in listings {
            if listing.seconds_remaining > 60 * 60 {
                continue;
            }

            let result = insert_listing(&*state, listing).await;
            if result.is_ok() {
                successful += 1;
            } else {
                eprintln!("{:#?}", result);
            }
        }

        Ok(format!("{}/{} updated", successful, total))
    }

    let route = warp::path("contribute")
        .and(warp::path("multiple"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(move |listings: Vec<PartyFinderListing>| logic(Arc::clone(&state), listings));
    warp::post().and(route).boxed()
}

async fn insert_listing(state: &State, listing: PartyFinderListing) -> Result<UpdateResult> {
    if listing.created_world < 1_000 || listing.home_world < 1_000 || listing.current_world < 1_000 {
        anyhow::bail!("invalid listing");
    }

    let opts = UpdateOptions::builder()
        .upsert(true)
        .build();
    let bson_value = mongodb::bson::to_bson(&listing).unwrap();
    let now = Utc::now();
    state
        .collection()
        .update_one(
            doc! {
                    "listing.id": listing.id,
                    "listing.last_server_restart": listing.last_server_restart,
                    "listing.created_world": listing.created_world as u32,
                },
            doc! {
                    "$currentDate": {
                        "updated_at": true,
                    },
                    "$set": {
                        "listing": bson_value,
                    },
                    "$setOnInsert": {
                        "created_at": now,
                    },
                },
            opts,
        )
        .await
        .context("could not insert record")
}

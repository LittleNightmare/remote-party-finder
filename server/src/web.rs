mod stats;

use std::{
    cmp::Ordering,
    convert::Infallible,
    sync::Arc,
    time::Duration,
};
use anyhow::{Result, Context};
use chrono::Utc;
use mongodb::{
    Client as MongoClient,
    Collection,
    IndexModel,
    bson::doc,
    options::{IndexOptions, UpdateOptions},
    results::UpdateResult,
};
use tokio::sync::RwLock;
use tokio_stream::StreamExt;
use warp::{
    Filter,
    Reply,
    filters::BoxedFilter,
    http::Uri,
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

pub async fn start(config: Arc<Config>) -> Result<()> {
    let state = State::new(Arc::clone(&config)).await?;

    println!("listening at {}", config.web.host);
    warp::serve(router(state))
        .run(config.web.host)
        .await;
    Ok(())
}

pub struct State {
    config: Arc<Config>,
    mongo: MongoClient,
    stats: RwLock<Option<CachedStatistics>>,
}

impl State {
    pub async fn new(config: Arc<Config>) -> Result<Arc<Self>> {
        let mongo = MongoClient::with_uri_str(&config.mongo.url)
            .await
            .context("could not create mongodb client")?;

        let state = Arc::new(Self {
            config,
            mongo,
            stats: Default::default(),
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

        Ok(state)
    }

    pub fn collection(&self) -> Collection<ListingContainer> {
        self.mongo.database("rpf").collection("listings")
    }
}

fn router(state: Arc<State>) -> BoxedFilter<(impl Reply, )> {
    index()
        .or(listings(Arc::clone(&state)))
        .or(contribute(Arc::clone(&state)))
        .or(contribute_multiple(Arc::clone(&state)))
        .or(stats(Arc::clone(&state)))
        .or(stats_seven_days(Arc::clone(&state)))
        .or(assets())
        .boxed()
}

fn assets() -> BoxedFilter<(impl Reply, )> {
    warp::get()
        .and(warp::path("assets"))
        .and(
            icons()
                .or(minireset())
                .or(common_css())
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

fn index() -> BoxedFilter<(impl Reply, )> {
    let route = warp::path::end()
        .map(|| warp::redirect(Uri::from_static("/listings")));
    warp::get().and(route).boxed()
}

fn list_js() -> BoxedFilter<(impl Reply, )> {
    warp::path("list.js")
        .and(warp::path::end())
        .and(warp::fs::file("./assets/list.min.js"))
        .boxed()
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
                            "time_left": { "$gte": 0 },
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

                Ok(ListingsTemplate {
                    containers,
                    lang,
                })
            }
            Err(e) => {
                eprintln!("{:#?}", e);
                Ok(ListingsTemplate {
                    containers: Default::default(),
                    lang,
                })
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

async fn insert_listing(state: &State, listing: PartyFinderListing) -> mongodb::error::Result<UpdateResult> {
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
}

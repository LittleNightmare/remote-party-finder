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
    bson::{doc, to_bson, Document},
    Client as MongoClient,
    Collection,
    IndexModel,
    options::{IndexOptions, UpdateOptions},
    results::UpdateResult,
};
use tokio::sync::RwLock;
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
    sestring_ext::SeStringExt,
    stats::CachedStatistics,
    template::listings::ListingsTemplate,
    template::stats::StatsTemplate,
};

mod stats;
pub mod api;
pub mod v2;

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
    entries: HashMap<u64, CacheEntry<DetailedApiListing>>,
}

const LISTING_ID_FIELD: &str = "listing.id";
const LISTING_LAST_SERVER_RESTART_FIELD: &str = "listing.last_server_restart";
const LISTING_CREATED_WORLD_FIELD: &str = "listing.created_world";
const ACTIVE_LISTING_WINDOW_HOURS: i64 = 2;

fn listing_identity_index_keys() -> Document {
    let mut keys = Document::new();
    keys.insert(LISTING_ID_FIELD, 1);
    keys.insert(LISTING_LAST_SERVER_RESTART_FIELD, 1);
    keys.insert(LISTING_CREATED_WORLD_FIELD, 1);
    keys
}

fn listing_id_index_keys() -> Document {
    let mut keys = Document::new();
    keys.insert(LISTING_ID_FIELD, 1);
    keys
}

fn listing_identity_filter(listing: &PartyFinderListing) -> Result<Document> {
    let mut filter = Document::new();
    filter.insert(LISTING_ID_FIELD, to_bson(&listing.id).context("could not serialize listing.id for upsert filter")?);
    filter.insert(
        LISTING_LAST_SERVER_RESTART_FIELD,
        to_bson(&listing.last_server_restart)
            .context("could not serialize listing.last_server_restart for upsert filter")?,
    );
    filter.insert(
        LISTING_CREATED_WORLD_FIELD,
        to_bson(&listing.created_world)
            .context("could not serialize listing.created_world for upsert filter")?,
    );
    Ok(filter)
}

fn active_listing_cutoff(now: DateTime<Utc>) -> DateTime<Utc> {
    // Legacy coexistence is bounded to the active-listing window; once a truncated historical row
    // ages out of this window, active reads stop surfacing it.
    now - chrono::Duration::hours(ACTIVE_LISTING_WINDOW_HOURS)
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
                    .keys(listing_identity_index_keys())
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
                    .keys(listing_id_index_keys())
                    .build(),
                None,
            )
            .await
            .context("could not create listing.id index")?;
            
        state.collection()
            .create_index(
                IndexModel::builder()
                    .keys(mongodb::bson::doc! {
                        "listing.category": 1,
                    })
                    .build(),
                None,
            )
            .await
            .context("could not create listing.category index")?;
            
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

        state.collection()
            .create_index(
                IndexModel::builder()
                    .keys(mongodb::bson::doc! {
                        "listing.duty": 1,
                    })
                    .build(),
                None,
            )
            .await
            .context("could not create listing.duty index")?;

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
    
    pub async fn get_detail_cache(&self, id: u64) -> Option<DetailedApiListing> {
        let cache = self.detail_cache.read().await;
        if let Some(entry) = cache.entries.get(&id) {
            if entry.expires_at > Utc::now() {
                return Some(entry.data.clone());
            }
        }
        None
    }
    
    pub async fn set_detail_cache(&self, id: u64, data: DetailedApiListing, ttl_seconds: i64) {
        let mut cache = self.detail_cache.write().await;
        let expires_at = Utc::now() + chrono::Duration::seconds(ttl_seconds);
        cache.entries.insert(id, CacheEntry { data, expires_at });
    }
}

pub(crate) fn router(state: Arc<State>) -> BoxedFilter<(impl Reply, )> {
    assets()
        .or(listings(Arc::clone(&state)))
        .or(stats(Arc::clone(&state)))
        .or(stats_seven_days(Arc::clone(&state)))
        .or(contribute(Arc::clone(&state)))
        .or(contribute_multiple(Arc::clone(&state)))
        .or(crate::web::api::listings_api(Arc::clone(&state)))
        .or(crate::web::api::listing_detail_api(Arc::clone(&state)))
        .or(crate::web::v2::routes(Arc::clone(&state)))
        .or(index())
        .boxed()
}

#[cfg(test)]
pub(crate) async fn state_for_router_tests() -> Arc<State> {
    let mongo = MongoClient::with_uri_str("mongodb://127.0.0.1:27017")
        .await
        .expect("router tests should construct a Mongo client without touching the network");

    Arc::new(State {
        mongo,
        stats: Default::default(),
        listings_cache: RwLock::new(ListingsCache {
            entries: HashMap::new(),
        }),
        detail_cache: RwLock::new(DetailCache {
            entries: HashMap::new(),
        }),
    })
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

        let two_hours_ago = active_listing_cutoff(Utc::now());
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
        let result = validate_and_insert_listing(&*state, listing).await;
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
            let result = validate_and_insert_listing(&*state, listing).await;
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

async fn validate_and_insert_listing(state: &State, listing: PartyFinderListing) -> Result<UpdateResult> {
    // Validate remaining time
    if listing.seconds_remaining > 60 * 60 {
        anyhow::bail!("invalid listing: remaining time greater than 1 hour");
    }

    if listing.last_server_restart < i64::from(i32::MIN)
        || listing.last_server_restart > i64::from(i32::MAX)
    {
        anyhow::bail!(
            "invalid listing: last_server_restart {} out of range (expected signed 32-bit integer)",
            listing.last_server_restart
        );
    }

    if !matches!(listing.created_world, 1000..=1999 | 4000..=4999) {
        anyhow::bail!(
            "invalid listing: created_world {} out of range (expected 1000-1999 or 4000-4999)",
            listing.created_world
        );
    }

    // Validate duty/category/duty_type combination (fast path, no allocation)
    match crate::ffxiv::is_valid_duty_combination(listing.duty_type, listing.category, listing.duty) {
        Ok(()) => {}
        Err(base_error) => {
            // Only extract strings when validation fails (lazy evaluation)
            let player_name = listing.name.full_text(&crate::ffxiv::Language::ChineseSimplified);
            let player_name = if player_name.is_empty() { None } else { Some(player_name.as_str()) };

            let world = listing.created_world();
            let world_str = world.map(|w| w.name());

            let description = listing.description.full_text(&crate::ffxiv::Language::ChineseSimplified);
            let description = if description.is_empty() { None } else { Some(description.as_str()) };

            // Build full error message with context
            let mut full_error = base_error;
            if let Some(name) = player_name {
                full_error.push_str(&format!(" | player: {}", name));
            }
            if let Some(w) = world_str {
                full_error.push_str(&format!(" | world: {}", w));
            }
            if let Some(desc) = description {
                let desc_preview: String = desc.chars().take(5).collect();
                full_error.push_str(&format!(" | desc: {}", desc_preview));
            }

            eprintln!("未插入: {}", full_error);
            anyhow::bail!("invalid listing: {}", full_error);
        }
    }

    insert_listing(state, listing).await
}

async fn insert_listing(state: &State, listing: PartyFinderListing) -> Result<UpdateResult> {
    let opts = UpdateOptions::builder()
        .upsert(true)
        .build();
    let bson_value = mongodb::bson::to_bson(&listing).unwrap();
    // Canonical writes always upsert on the widened `(listing.id, last_server_restart,
    // created_world)` identity only. Pre-migration truncated-id rows remain legacy data; the
    // server does not reconstruct guessed wide ids from `content_id_lower` or any other surrogate.
    let filter = listing_identity_filter(&listing)?;
    let now = Utc::now();
    state
        .collection()
        .update_one(
            filter,
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

#[cfg(test)]
mod tests {
    use super::*;

    const WIDE_LISTING_ID: u64 = 4_294_967_296;

    const LISTING_FIXTURE: &str = r###"
{
  "id": 4294967419,
  "content_id_lower": 456,
  "name": "VGVzdCBOYW1l",
  "description": "VGhpcyBpcyBteSB0ZXN0IGRlc2NyaXB0aW9uLg==",
  "created_world": 1001,
  "home_world": 1001,
  "current_world": 1001,
  "category": 0,
  "duty": 55,
  "duty_type": 2,
  "beginners_welcome": false,
  "seconds_remaining": 3300,
  "min_item_level": 0,
  "num_parties": 1,
  "slots_available": 7,
  "last_server_restart": 1234567890,
  "objective": 3,
  "conditions": 1,
  "duty_finder_settings": 0,
  "loot_rules": 0,
  "search_area": 1,
  "slots": [
    {
      "accepting": 167772160
    }
  ],
  "jobs_present": [
    5,
    0,
    0,
    0,
    0,
    0,
    0,
    0
  ]
}"###;

    fn fixture_listing() -> PartyFinderListing {
        serde_json::from_str(LISTING_FIXTURE).expect("listing fixture must deserialize")
    }

    fn wide_listing_fixture(content_id_lower: u32) -> PartyFinderListing {
        let mut listing = fixture_listing();
        listing.id = WIDE_LISTING_ID;
        listing.content_id_lower = content_id_lower;
        listing
    }

    fn valid_upload_listing() -> PartyFinderListing {
        let mut listing = fixture_listing();
        listing.duty = 0;
        listing
    }

    fn detail_cache_fixture() -> DetailedApiListing {
        DetailedApiListing {
            id: WIDE_LISTING_ID,
            name: "Wide Test Name".to_string(),
            description: "Wide test description".to_string(),
            created_world: "Test World".to_string(),
            home_world: "Test World".to_string(),
            category: "None".to_string(),
            duty: "Test Duty".to_string(),
            min_item_level: 0,
            slots_filled: 0,
            slots_available: 8,
            time_left: 42.0,
            updated_at: "2026-05-03T00:00:00Z".to_string(),
            is_cross_world: false,
            beginners_welcome: false,
            duty_type: "Normal".to_string(),
            objective: "Practice".to_string(),
            conditions: "None".to_string(),
            loot_rules: "None".to_string(),
            slots: vec![crate::web::api::SlotInfo {
                filled: false,
                role: None,
                role_id: 0,
                job: None,
                job_id: vec![],
            }],
            datacenter: Some("Test DC".to_string()),
        }
    }

    #[test]
    fn listing_identity_filter_keeps_legacy_rows_non_reconstructable() {
        let listing = fixture_listing();

        let filter = listing_identity_filter(&listing).expect("filter must serialize");

        assert_eq!(filter.len(), 3);
        assert_eq!(filter.get(LISTING_ID_FIELD), Some(&to_bson(&listing.id).unwrap()));
        assert_eq!(
            filter.get(LISTING_LAST_SERVER_RESTART_FIELD),
            Some(&to_bson(&listing.last_server_restart).unwrap()),
        );
        assert_eq!(
            filter.get(LISTING_CREATED_WORLD_FIELD),
            Some(&to_bson(&listing.created_world).unwrap()),
        );
        assert!(!filter.contains_key("listing.content_id_lower"));
    }

    #[test]
    fn active_listing_cutoff_bounds_temporary_legacy_row_coexistence_window() {
        let now = Utc::now();

        assert_eq!(
            active_listing_cutoff(now),
            now - chrono::Duration::hours(ACTIVE_LISTING_WINDOW_HOURS),
        );
    }

    #[tokio::test]
    async fn detail_cache_round_trips_wide_key_exactly() {
        let state = state_for_router_tests().await;
        let listing = detail_cache_fixture();

        state.set_detail_cache(WIDE_LISTING_ID, listing.clone(), 60).await;

        let cached = state
            .get_detail_cache(WIDE_LISTING_ID)
            .await
            .expect("wide cache key must round-trip exactly");
        assert_eq!(
            serde_json::to_value(cached).expect("cached listing must serialize"),
            serde_json::to_value(listing).expect("fixture must serialize"),
        );
        assert!(state.get_detail_cache(WIDE_LISTING_ID - 1).await.is_none());
    }

#[test]
    fn listing_identity_index_keys_has_three_canonical_fields() {
        let keys = listing_identity_index_keys();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains_key(LISTING_ID_FIELD));
        assert!(keys.contains_key(LISTING_LAST_SERVER_RESTART_FIELD));
        assert!(keys.contains_key(LISTING_CREATED_WORLD_FIELD));
    }

    #[test]
    fn listing_id_index_keys_has_single_field() {
        let keys = listing_id_index_keys();
        assert_eq!(keys.len(), 1);
        assert!(keys.contains_key(LISTING_ID_FIELD));
    }

    #[test]
    fn canonical_upsert_filter_is_independent_of_non_key_fields() {
        let first = wide_listing_fixture(456);
        let mut second = wide_listing_fixture(789);
        second.seconds_remaining = first.seconds_remaining - 1;

        let first_filter = listing_identity_filter(&first).expect("first filter must build");
        let second_filter = listing_identity_filter(&second).expect("second filter must build");

        assert_eq!(first_filter, second_filter);
        assert_eq!(first_filter.len(), 3);
        assert!(!first_filter.contains_key("listing.content_id_lower"));
    }

    #[tokio::test]
    async fn contribute_multiple_accepts_wide_duplicate_payload_before_insert_io_failure() {
        let state = state_for_router_tests().await;
        let route = contribute_multiple(Arc::clone(&state));
        let payload = vec![wide_listing_fixture(456), wide_listing_fixture(789)];

        let response = warp::test::request()
            .method("POST")
            .path("/contribute/multiple")
            .json(&payload)
            .reply(&route)
            .await;

        assert_eq!(response.status(), warp::http::StatusCode::OK);
        assert_eq!(std::str::from_utf8(response.body()).unwrap(), "0/2 updated");
    }

    #[tokio::test]
    async fn validate_and_insert_listing_accepts_signed_i32_boundary_values() {
        let state = state_for_router_tests().await;

        for boundary in [i64::from(i32::MIN), i64::from(i32::MAX)] {
            let mut listing = valid_upload_listing();
            listing.last_server_restart = boundary;

            let error = validate_and_insert_listing(&state, listing)
                .await
                .expect_err("fixture should reach insert path and fail on io");

            assert!(
                error.to_string().contains("could not insert record"),
                "expected insert io failure for boundary {boundary}, got: {error:#}"
            );
        }
    }

    #[tokio::test]
    async fn validate_and_insert_listing_rejects_values_outside_signed_i32_range() {
        let state = state_for_router_tests().await;

        for out_of_range in [i64::from(i32::MIN) - 1, i64::from(i32::MAX) + 1] {
            let mut listing = valid_upload_listing();
            listing.last_server_restart = out_of_range;

            let error = validate_and_insert_listing(&state, listing)
                .await
                .expect_err("out-of-range restart timestamp must be rejected");

            assert!(
                error
                    .to_string()
                    .contains("invalid listing: last_server_restart"),
                "expected validation failure for {out_of_range}, got: {error:#}"
            );
        }
    }
}

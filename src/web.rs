use std::cmp::Ordering;
use std::convert::{Infallible, TryFrom};
use anyhow::{Result, Context};
use std::sync::Arc;
use mongodb::{Client as MongoClient, Collection, IndexModel};
use mongodb::options::{IndexOptions, UpdateOptions};
use mongodb::results::UpdateResult;
use tokio_stream::StreamExt;
use warp::{Filter, Reply};
use warp::filters::BoxedFilter;
use warp::http::Uri;
use crate::config::Config;
use crate::listing::PartyFinderListing;
use crate::listing_container::{ListingContainer, QueriedListing};
use crate::template::listings::ListingsTemplate;

pub async fn start(config: Arc<Config>) -> Result<()> {
    let state = State::new(Arc::clone(&config)).await?;

    println!("listening at {}", config.web.host);
    warp::serve(router(state))
        .run(config.web.host)
        .await;
    Ok(())
}

struct State {
    config: Arc<Config>,
    mongo: MongoClient,
}

impl State {
    pub async fn new(config: Arc<Config>) -> Result<Arc<Self>> {
        let mongo = MongoClient::with_uri_str(&config.mongo.url)
            .await
            .context("could not create mongodb client")?;

        let state = Arc::new(Self {
            config,
            mongo,
        });

        state.collection()
            .create_index(
                IndexModel::builder()
                    .keys(mongodb::bson::doc! {
                        "listing.id": 1,
                        "listing.content_id_lower": 1,
                    })
                    .options(IndexOptions::builder()
                        .unique(true)
                        .build())
                    .build(),
                None,
            )
            .await
            .context("could not create index")?;

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
        .or(assets())
        .boxed()
}

fn assets() -> BoxedFilter<(impl Reply, )> {
    warp::get()
        .and(warp::path("assets"))
        .and(
            icons()
                .or(minireset())
                .or(listings_css())
                .or(listings_js())
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

fn index() -> BoxedFilter<(impl Reply, )> {
    let route = warp::path::end()
        .map(|| warp::redirect(Uri::from_static("/listings")));
    warp::get().and(route).boxed()
}

fn listings(state: Arc<State>) -> BoxedFilter<(impl Reply, )> {
    async fn logic(state: Arc<State>) -> std::result::Result<impl Reply, Infallible> {
        use mongodb::bson::doc;

        let res = state
            .collection()
            .aggregate(
                [
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
                            "time_left": {
                                "$gte": 0,
                            },
                        }
                    },
                    doc! {
                        "$sort": {
                            "updated_minute": -1,
                            "time_left": 1,
                        }
                    }
                ],
                None,
            )
            .await;
        Ok(match res {
            Ok(mut cursor) => {
                let mut containers = Vec::new();

                while let Ok(Some(container)) = cursor.try_next().await {
                    let res: Result<QueriedListing> = try {
                        let json = serde_json::to_vec(&container)?;
                        let result: QueriedListing = serde_json::from_slice(&json)?;
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
                })
            }
            Err(e) => {
                eprintln!("{:#?}", e);
                Ok(ListingsTemplate {
                    containers: Default::default(),
                })
            }
        })
    }

    let route = warp::path("listings")
        .and(warp::path::end())
        .and_then(move || logic(Arc::clone(&state)));

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
    use mongodb::bson::doc;

    let opts = UpdateOptions::builder()
        .upsert(true)
        .build();
    let value = serde_json::to_value(&listing).unwrap();
    let bson_value = mongodb::bson::Bson::try_from(value).unwrap();
    state
        .collection()
        .update_one(
            doc! {
                    "listing.id": listing.id,
                    "listing.content_id_lower": listing.content_id_lower,
                },
            doc! {
                    "$currentDate": {
                        "updated_at": true,
                    },
                    "$set": {
                        "listing": bson_value,
                    }
                },
            opts,
        )
        .await
}

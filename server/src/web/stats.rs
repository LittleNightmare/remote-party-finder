use anyhow::Result;
use mongodb::bson::{Document, doc};
use tokio_stream::StreamExt;
use crate::stats::Statistics;
use crate::web::State;

lazy_static::lazy_static! {
    static ref QUERY: [Document; 1] = [
        doc! {
            "$facet": {
                "count": [
                    {
                        "$count": "count",
                    },
                ],
                "aliases": [
                    {
                        "$group": {
                            "_id": "$listing.content_id_lower",
                            "aliases": {
                                "$addToSet": {
                                    "name": "$listing.name",
                                    "home_world": "$listing.home_world",
                                },
                            },
                        }
                    }
                ],
                "duties": [
                    {
                        "$group": {
                            "_id": [
                                "$listing.duty_type",
                                "$listing.category",
                                "$listing.duty",
                            ],
                            "count": {
                                "$sum": 1
                            },
                        }
                    },
                    {
                        "$sort": {
                            "count": -1,
                        }
                    }
                ],
                "hosts": [
                    {
                        "$group": {
                            "_id": "$listing.content_id_lower",
                            "count": {
                                "$sum": 1
                            },
                        }
                    },
                    {
                        "$sort": {
                            "count": -1
                        }
                    },
                    {
                        "$limit": 15
                    }
                ],
                "hours": [
                    {
                        "$group": {
                            "_id": {
                                "$hour": "$created_at",
                            },
                            "count": {
                                "$sum": 1
                            },
                        }
                    },
                    {
                        "$sort": {
                            "_id": 1,
                        }
                    }
                ],
                "days": [
                    {
                        "$group": {
                            "_id": {
                                "$dayOfWeek": "$created_at",
                            },
                            "count": {
                                "$sum": 1
                            },
                        }
                    },
                    {
                        "$sort": {
                            "_id": 1,
                        }
                    }
                ],
            }
        },
    ];
}

pub async fn get_stats(state: &State) -> Result<Statistics> {
    let mut cursor = state
        .collection()
        .aggregate(QUERY.iter().cloned(), None)
        .await?;
    let doc = cursor.try_next().await?;
    let doc = doc.ok_or_else(|| anyhow::anyhow!("missing document"))?;
    let stats = mongodb::bson::from_document(doc)?;
    Ok(stats)
}

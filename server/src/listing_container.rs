use chrono::{DateTime, Duration, Utc};
use chrono_humanize::HumanTime;
use serde::{Deserialize, Serialize};
use crate::listing::PartyFinderListing;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ListingContainer {
    #[serde(with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
    pub listing: PartyFinderListing,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct QueriedListing {
    #[serde(with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
    #[serde(with = "mongodb::bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_minute: DateTime<Utc>,
    pub time_left: f64,
    pub listing: PartyFinderListing,
}

impl QueriedListing {
    pub fn human_time_left(&self) -> HumanTime {
        HumanTime::from(Duration::milliseconds((self.time_left * 1000f64) as i64))
    }

    pub fn since_updated(&self) -> Duration {
        Utc::now() - self.updated_at
    }

    pub fn human_since_updated(&self) -> HumanTime {
        HumanTime::from(-self.since_updated())
    }
}

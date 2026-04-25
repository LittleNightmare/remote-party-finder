use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pagination {
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
    pub total_pages: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CollectionEnvelope<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemberEnvelope<T> {
    pub data: T,
}

pub type ListingCollectionResponse = CollectionEnvelope<ListingSummary>;
pub type ListingMemberResponse = MemberEnvelope<ListingDetail>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorEnvelope {
    pub error: ErrorBody,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
    pub details: Map<String, Value>,
}

impl ErrorEnvelope {
    pub fn new(
        code: impl Into<String>,
        message: impl Into<String>,
        details: Map<String, Value>,
    ) -> Self {
        Self {
            error: ErrorBody {
                code: code.into(),
                message: message.into(),
                details,
            },
        }
    }

    pub fn invalid_query(field: impl Into<String>, message: impl Into<String>) -> Self {
        let field = field.into();
        let message = message.into();
        let mut details = Map::new();
        details.insert("field".into(), Value::String(field));

        Self::new("invalid_query", message, details)
    }

    pub fn not_implemented(message: impl Into<String>) -> Self {
        Self::new("not_implemented", message, Map::new())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListingSummary {
    pub id: u32,
    pub player_name: String,
    pub description: String,
    pub created_world_id: u32,
    pub home_world_id: u32,
    pub category_id: u32,
    pub duty_id: u32,
    pub duty_type_id: u32,
    pub min_item_level: u16,
    pub slots_filled: usize,
    pub slots_available: u8,
    pub time_left_seconds: u32,
    pub updated_at: String,
    pub is_cross_world: bool,
    pub beginners_welcome: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListingDetail {
    pub id: u32,
    pub player_name: String,
    pub description: String,
    pub created_world_id: u32,
    pub home_world_id: u32,
    pub category_id: u32,
    pub duty_id: u32,
    pub duty_type_id: u32,
    pub min_item_level: u16,
    pub slots_filled: usize,
    pub slots_available: u8,
    pub time_left_seconds: u32,
    pub updated_at: String,
    pub is_cross_world: bool,
    pub beginners_welcome: bool,
    pub objective_ids: Vec<u32>,
    pub condition_ids: Vec<u32>,
    pub loot_rule_id: u32,
    pub slots: Vec<ListingSlot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListingSlot {
    pub filled: bool,
    pub role_id: u32,
    pub filled_job_id: Option<u32>,
    pub accepted_job_ids: Vec<u32>,
}

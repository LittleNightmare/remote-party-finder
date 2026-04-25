use crate::listing::{
    ConditionFlags, DutyCategory, DutyFinderSettingsFlags, DutyType, JobFlags, LootRuleFlags,
    ObjectiveFlags, PartyFinderListing, PartyFinderSlot, SearchAreaFlags,
};
use sestring::SeString;

const LISTING: &str = r###"
{
  "id": 123,
  "content_id_lower": 456,
  "name": "VGVzdCBOYW1l",
  "description": "VGhpcyBpcyBteSB0ZXN0IGRlc2NyaXB0aW9uLg==",
  "created_world": 73,
  "home_world": 73,
  "current_world": 73,
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

lazy_static::lazy_static! {
    static ref EXPECTED: PartyFinderListing = PartyFinderListing {
        id: 123,
        content_id_lower: 456,
        name: SeString::parse(b"Test Name").unwrap(),
        description: SeString::parse(b"This is my test description.").unwrap(),
        created_world: 73,
        home_world: 73,
        current_world: 73,
        category: DutyCategory::None,
        last_server_restart: 1234567890,
        duty: 55,
        duty_type: DutyType::Normal,
        beginners_welcome: false,
        seconds_remaining: 3300,
        min_item_level: 0,
        num_parties: 1,
        slots_available: 7,
        objective: ObjectiveFlags::PRACTICE | ObjectiveFlags::DUTY_COMPLETION,
        conditions: ConditionFlags::NONE,
        duty_finder_settings: DutyFinderSettingsFlags::NONE,
        loot_rules: LootRuleFlags::NONE,
        search_area: SearchAreaFlags::DATA_CENTRE,
        slots: vec![
            PartyFinderSlot {
                accepting: JobFlags::DANCER | JobFlags::BLUE_MAGE,
            },
        ],
        jobs_present: vec![5, 0, 0, 0, 0, 0, 0, 0],
    };
}

#[test]
fn deserialise_listing() {
    let listing: PartyFinderListing = serde_json::from_str(LISTING).unwrap();
    assert_eq!(listing, *EXPECTED,)
}

#[test]
fn serialise_listing() {
    assert_eq!(
        serde_json::to_string_pretty(&*EXPECTED).unwrap(),
        LISTING.trim(),
    );
}

mod api_v1_regression {
    use super::*;

    #[test]
    fn listings_shape_unchanged() {
        const V1_COLLECTION_JSON: &str = r###"{
  "data": [
    {
      "id": 123,
      "name": "Test Player",
      "description": "Test description",
      "created_world": "Cerberus",
      "created_world_id": 73,
      "home_world": "Cerberus",
      "home_world_id": 73,
      "category": "Dungeons",
      "category_id": 1,
      "duty": "The Navel",
      "min_item_level": 0,
      "slots_filled": 1,
      "slots_available": 7,
      "time_left": 3000.5,
      "updated_at": "2024-01-01T00:00:00Z",
      "is_cross_world": true,
      "datacenter": "Gaia"
    }
  ],
  "pagination": {
    "total": 1,
    "page": 1,
    "per_page": 20,
    "total_pages": 1
  }
}"###;

        let value: serde_json::Value =
            serde_json::from_str(V1_COLLECTION_JSON).expect("must parse");

        assert!(value.get("data").is_some(), "must have data field");
        assert!(
            value.get("pagination").is_some(),
            "must have pagination field"
        );

        let pagination = value.get("pagination").unwrap().as_object().unwrap();
        assert!(pagination.contains_key("total"));
        assert!(pagination.contains_key("page"));
        assert!(pagination.contains_key("per_page"));
        assert!(pagination.contains_key("total_pages"));

        let listing = value["data"].as_array().unwrap().first().unwrap();
        assert!(listing.get("id").is_some());
        assert!(listing.get("name").is_some());
        assert!(listing.get("created_world").is_some());
        assert!(listing.get("category").is_some());
        assert!(listing.get("duty").is_some());
    }

    #[test]
    fn listing_detail_shape_unchanged() {
        const V1_DETAIL_JSON: &str = r###"{
  "id": 123,
  "name": "Test Player",
  "description": "Test description",
  "created_world": "Cerberus",
  "home_world": "Cerberus",
  "category": "Dungeons",
  "duty": "The Navel",
  "min_item_level": 0,
  "slots_filled": 1,
  "slots_available": 7,
  "time_left": 3000.5,
  "updated_at": "2024-01-01T00:00:00Z",
  "is_cross_world": true,
  "beginners_welcome": false,
  "duty_type": "Normal",
  "objective": "Boss",
  "conditions": "None",
  "loot_rules": "Normal",
  "slots": [
    {
      "filled": true,
      "role": "DPS",
      "role_id": 3,
      "job": "Ninja",
      "job_id": [34]
    }
  ],
  "datacenter": "Gaia"
}"###;

        let value: serde_json::Value = serde_json::from_str(V1_DETAIL_JSON).expect("must parse");

        assert_eq!(value.get("id").unwrap().as_u64().unwrap(), 123);
        assert!(value.get("name").is_some());
        assert!(value.get("created_world").is_some());
        assert!(value.get("home_world").is_some());
        assert!(value.get("category").is_some());
        assert!(value.get("duty").is_some());
        assert!(value.get("slots_available").is_some());
        assert!(value.get("slots").is_some());

        assert!(value.get("beginners_welcome").is_some());
        assert!(value.get("duty_type").is_some());
        assert!(value.get("objective").is_some());
        assert!(value.get("conditions").is_some());
        assert!(value.get("loot_rules").is_some());
    }

    #[test]
    fn contribute_multiple_shape_unchanged() {
        let payload = format!("[{}]", LISTING.trim());
        let listings: Vec<PartyFinderListing> = serde_json::from_str(&payload).expect("must parse");

        assert_eq!(listings.len(), 1);
        assert_eq!(listings.first().unwrap(), &*EXPECTED);

        let actual = serde_json::to_value(&listings).expect("must serialize");
        let expected: serde_json::Value =
            serde_json::from_str(&payload).expect("payload fixture must parse");
        assert_eq!(actual, expected);
    }
}

use crate::listing::{
    ConditionFlags, DutyCategory, DutyFinderSettingsFlags, DutyType, JobFlags, LootRuleFlags,
    ObjectiveFlags, PartyFinderListing, PartyFinderSlot, SearchAreaFlags,
};
use sestring::SeString;

const NARROW_LISTING_ID: u64 = 123;
const WIDE_LISTING_ID: u64 = 4_294_967_296;

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
        id: NARROW_LISTING_ID,
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

fn listing_json_with_id(id: u64) -> String {
    LISTING.replacen(
        &format!("\"id\": {NARROW_LISTING_ID},"),
        &format!("\"id\": {id},"),
        1,
    )
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

#[test]
fn wide_listing_id_round_trips_exactly() {
    let fixture = listing_json_with_id(WIDE_LISTING_ID);
    let listing: PartyFinderListing = serde_json::from_str(&fixture).unwrap();

    assert_eq!(listing.id, WIDE_LISTING_ID);
    assert_eq!(
        serde_json::to_string_pretty(&listing).unwrap(),
        fixture.trim()
    );
}

mod api_v1_regression {
    use super::*;

    const LEGACY_WRAPPED_RESTART_TIMESTAMP: i64 = 4_294_967_295;

    fn listing_json_with_last_server_restart(last_server_restart: i64) -> String {
        LISTING.replacen(
            "\"last_server_restart\": 1234567890,",
            &format!("\"last_server_restart\": {last_server_restart},"),
            1,
        )
    }

    #[test]
    fn contribute_multiple_accepts_mixed_and_duplicate_wide_ids() {
        let wide_listing = listing_json_with_id(WIDE_LISTING_ID);
        let payload = format!(
            "[{},{},{}]",
            LISTING.trim(),
            wide_listing.trim(),
            wide_listing.trim()
        );
        let listings: Vec<PartyFinderListing> = serde_json::from_str(&payload).expect("must parse");

        assert_eq!(listings.len(), 3);
        assert_eq!(
            listings
                .iter()
                .map(|listing| listing.id)
                .collect::<Vec<_>>(),
            vec![NARROW_LISTING_ID, WIDE_LISTING_ID, WIDE_LISTING_ID]
        );

        let actual = serde_json::to_value(&listings).expect("must serialize");
        let expected: serde_json::Value =
            serde_json::from_str(&payload).expect("payload fixture must parse");
        assert_eq!(actual, expected);
    }

    #[test]
    fn negative_last_server_restart_round_trips_exactly() {
        let fixture = listing_json_with_last_server_restart(-1);
        let listing: PartyFinderListing = serde_json::from_str(&fixture).expect("must parse");

        assert_eq!(listing.last_server_restart, -1);
        assert_eq!(
            serde_json::to_string_pretty(&listing).expect("must serialize"),
            fixture.trim()
        );
    }

    #[test]
    fn legacy_wrapped_restart_timestamp_round_trips_for_bridge_storage() {
        let fixture = listing_json_with_last_server_restart(LEGACY_WRAPPED_RESTART_TIMESTAMP);
        let listing: PartyFinderListing = serde_json::from_str(&fixture).expect("must parse");

        assert_eq!(
            listing.last_server_restart,
            LEGACY_WRAPPED_RESTART_TIMESTAMP
        );
        assert_eq!(
            serde_json::to_string_pretty(&listing).expect("must serialize"),
            fixture.trim()
        );
    }

    #[test]
    fn contribute_multiple_preserves_negative_last_server_restart_in_payload() {
        let negative_restart_listing = listing_json_with_last_server_restart(-123);
        let payload = format!("[{}]", negative_restart_listing.trim());
        let listings: Vec<PartyFinderListing> = serde_json::from_str(&payload).expect("must parse");

        assert_eq!(listings.len(), 1);
        assert_eq!(listings[0].last_server_restart, -123);

        let actual = serde_json::to_value(&listings).expect("must serialize");
        assert_eq!(actual[0]["last_server_restart"], -123);
    }
}

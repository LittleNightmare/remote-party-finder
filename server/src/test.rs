use crate::listing::{
    ConditionFlags, DutyCategory, DutyFinderSettingsFlags, DutyType, JobFlags, LootRuleFlags,
    ObjectiveFlags, PartyFinderListing, PartyFinderSlot, SearchAreaFlags,
};
use ffxiv_types_cn::jobs::{ClassJob, Job};
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

mod beastmaster_mapping {
    use super::*;

    #[test]
    fn beastmaster_mapping_uses_canonical_slot_bits_for_public_job_ids() {
        assert_eq!(
            JobFlags::accepted_slot_bit_for_job_id(19),
            Some(u64::from(JobFlags::PALADIN.bits()))
        );
        assert_eq!(JobFlags::accepted_slot_bit_for_job_id(43), Some(1u64 << 32));
    }

    #[test]
    fn beastmaster_mapping_uses_canonical_slot_bits_for_public_job_codes() {
        assert_eq!(
            JobFlags::accepted_slot_bit_for_job_code("PLD"),
            Some(u64::from(JobFlags::PALADIN.bits()))
        );
        assert_eq!(
            JobFlags::accepted_slot_bit_for_job_code("BST"),
            Some(1u64 << 32)
        );
        assert_eq!(JobFlags::accepted_slot_bit_for_job_id(8), None);
    }

    #[test]
    fn beastmaster_slot_mask_round_trips_exactly() {
        let slot = PartyFinderSlot {
            accepting: JobFlags::BEASTMASTER,
        };
        let json = serde_json::to_string_pretty(&slot).unwrap();
        let decoded: PartyFinderSlot = serde_json::from_str(&json).unwrap();
        assert_eq!(slot.accepting, decoded.accepting);
        assert!(decoded.accepting.contains(JobFlags::BEASTMASTER));
    }

    #[test]
    fn classjobs_decodes_beastmaster() {
        let slot = PartyFinderSlot {
            accepting: JobFlags::BEASTMASTER,
        };
        let cjs = slot.accepting.classjobs();
        assert_eq!(cjs.len(), 1);
        assert_eq!(cjs[0], ClassJob::Job(Job::Beastmaster));
    }
}

mod beastmaster_web_compat {
    use std::{fs, path::Path};

    #[test]
    fn listings_icon_sheet_contains_bst_symbol() {
        let icons =
            fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/icons.svg"))
                .expect("must read icons asset");
        assert_eq!(icons.matches(r#"id="BST""#).count(), 1);

        let listings_template = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("templates/listings.html"),
        )
        .expect("must read listings template");
        assert!(
            listings_template.contains(r#"<use href="/assets/icons.svg#{{ title }}"></use>"#),
            "listings template must keep existing icon contract"
        );
    }

    #[test]
    fn api_v1_docs_mention_beastmaster_bst_and_43() {
        let docs =
            fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/api-v1.md"))
                .expect("must read v1 api docs");
        let expected_empty_slot_job_ids = "\"job_id\": [1, 2, 3, 4, 5, 6, 7, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43]";

        assert!(docs.contains("Beastmaster"));
        assert!(docs.contains("BST"));
        assert!(docs.contains("43"));
        assert!(!docs.contains("\"job_id\": [1, 2, 3]"));
        assert_eq!(docs.matches(expected_empty_slot_job_ids).count(), 7);
        assert_eq!(docs.matches("\"role_id\": 0,").count(), 7);
    }
}

mod legacy_storage {
    use super::*;

    #[test]
    fn legacy_job_mask_round_trips_after_u64_upgrade() {
        let legacy_json = r#"{"accepting": 167772160}"#;
        let slot: PartyFinderSlot = serde_json::from_str(legacy_json).unwrap();
        assert!(slot.accepting.contains(JobFlags::DANCER));
        assert!(slot.accepting.contains(JobFlags::BLUE_MAGE));
    }
}

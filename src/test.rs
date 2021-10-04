use crate::listing::{PartyFinderListing, DutyType, ObjectiveFlags, ConditionFlags, DutyFinderSettingsFlags, LootRuleFlags, SearchAreaFlags, DutyCategory, PartyFinderSlot, JobFlags};
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
        category: DutyCategory::Duty,
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
    assert_eq!(
        listing,
        *EXPECTED,
    )
}

#[test]
fn serialise_listing() {
    assert_eq!(
        serde_json::to_string_pretty(&*EXPECTED).unwrap(),
        LISTING.trim(),
    );
}

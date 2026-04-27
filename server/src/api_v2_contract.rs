use serde_json::json;
use warp::http::StatusCode;
use mongodb::bson::doc;

use crate::listing::{
    ConditionFlags, DutyCategory, DutyType, JobFlags, LootRuleFlags, ObjectiveFlags,
};
use crate::listing_container::QueriedListing;
use crate::web::v2::contracts::{
    CollectionEnvelope, ErrorEnvelope, ListingDetail, ListingMemberResponse, ListingSlot,
    ListingSummary, Pagination,
};
use crate::web::v2::filters::ListingsQuery;
use crate::web::v2::id_inventory;
use crate::web::v2::listings::{
    collection_response_from_documents,
    collection_response_from_raw_documents_for_tests, member_route_for_tests,
    project_listing_detail, project_listing_summaries, project_listing_summary,
    resolve_listing_detail,
};
use chrono::{Duration, Utc};

const ACTIVE_FIXTURE_JSON: &str = r#"{
  "id": 12345,
  "content_id_lower": 456,
  "name": "QWN0aXZl",
  "description": "QWN0aXZlIGZpeHR1cmU=",
  "created_world": 73,
  "home_world": 73,
  "current_world": 73,
  "category": 64,
  "last_server_restart": 1000,
  "duty": 55,
  "duty_type": 2,
  "beginners_welcome": false,
  "seconds_remaining": 1200,
  "min_item_level": 0,
  "num_parties": 1,
  "slots_available": 8,
  "objective": 3,
  "conditions": 1,
  "duty_finder_settings": 0,
  "loot_rules": 0,
  "search_area": 1,
  "slots": [{ "accepting": 167772160 }],
  "jobs_present": [5, 0, 0, 0, 0, 0, 0, 0]
}"#;

const DUPLICATE_OLD_FIXTURE_JSON: &str = r#"{
  "id": 12345,
  "content_id_lower": 457,
  "name": "RHVwbGljYXRl",
  "description": "T2xkIGR1cGxpY2F0ZSBmaXh0dXJl",
  "created_world": 73,
  "home_world": 73,
  "current_world": 73,
  "category": 64,
  "last_server_restart": 999,
  "duty": 55,
  "duty_type": 2,
  "beginners_welcome": false,
  "seconds_remaining": 900,
  "min_item_level": 0,
  "num_parties": 1,
  "slots_available": 8,
  "objective": 3,
  "conditions": 1,
  "duty_finder_settings": 0,
  "loot_rules": 0,
  "search_area": 1,
  "slots": [{ "accepting": 167772160 }],
  "jobs_present": [5, 0, 0, 0, 0, 0, 0, 0]
}"#;

const CROSS_WORLD_FIXTURE_JSON: &str = r#"{
  "id": 12346,
  "content_id_lower": 458,
  "name": "Q3Jvc3MgV29ybGQ=",
  "description": "Q3Jvc3Mtd29ybGQgZml4dHVyZQ==",
  "created_world": 73,
  "home_world": 73,
  "current_world": 73,
  "category": 64,
  "last_server_restart": 1000,
  "duty": 55,
  "duty_type": 2,
  "beginners_welcome": false,
  "seconds_remaining": 1200,
  "min_item_level": 0,
  "num_parties": 1,
  "slots_available": 8,
  "objective": 3,
  "conditions": 1,
  "duty_finder_settings": 0,
  "loot_rules": 0,
  "search_area": 1,
  "slots": [{ "accepting": 167772160 }],
  "jobs_present": [5, 0, 0, 0, 0, 0, 0, 0]
}"#;

const EXPIRED_FIXTURE_JSON: &str = r#"{
  "id": 12345,
  "content_id_lower": 459,
  "name": "RXhwaXJlZA==",
  "description": "RXhwaXJlZCBmaXh0dXJl",
  "created_world": 73,
  "home_world": 73,
  "current_world": 73,
  "category": 64,
  "last_server_restart": 1000,
  "duty": 55,
  "duty_type": 2,
  "beginners_welcome": false,
  "seconds_remaining": 0,
  "min_item_level": 0,
  "num_parties": 1,
  "slots_available": 8,
  "objective": 3,
  "conditions": 1,
  "duty_finder_settings": 0,
  "loot_rules": 0,
  "search_area": 1,
  "slots": [{ "accepting": 167772160 }],
  "jobs_present": [5, 0, 0, 0, 0, 0, 0, 0]
}"#;

#[test]
fn success_envelopes_are_consistent() {
    let collection = CollectionEnvelope {
        data: vec![sample_summary()],
        pagination: Pagination {
            total: 1,
            page: 1,
            per_page: 20,
            total_pages: 1,
        },
    };

    assert_eq!(
        serde_json::to_value(&collection).unwrap(),
        json!({
            "data": [
                {
                    "id": 900001,
                    "player_name": "Alice",
                    "description": "Need clear",
                    "created_world_id": 1167,
                    "home_world_id": 1167,
                    "category_id": 64,
                    "duty_id": 1234,
                    "duty_type_id": 0,
                    "min_item_level": 710,
                    "slots_filled": 6,
                    "slots_available": 8,
                    "time_left_seconds": 1200,
                    "updated_at": "2026-04-23T12:34:56Z",
                    "is_cross_world": true,
                    "beginners_welcome": false
                }
            ],
            "pagination": {
                "total": 1,
                "page": 1,
                "per_page": 20,
                "total_pages": 1
            }
        })
    );

    let member = ListingMemberResponse {
        data: sample_detail(),
    };

    assert_eq!(
        serde_json::to_value(&member).unwrap(),
        json!({
            "data": {
                "id": 900001,
                "player_name": "Alice",
                "description": "Need clear",
                "created_world_id": 1167,
                "home_world_id": 1167,
                "category_id": 64,
                "duty_id": 1234,
                "duty_type_id": 0,
                "min_item_level": 710,
                "slots_filled": 6,
                "slots_available": 8,
                "time_left_seconds": 1200,
                "updated_at": "2026-04-23T12:34:56Z",
                "is_cross_world": true,
                "beginners_welcome": false,
                "objective_ids": [1, 4],
                "condition_ids": [2],
                "loot_rule_id": 3,
                "slots": [
                    {
                        "filled": true,
                        "role_id": 3,
                        "filled_job_id": 19,
                        "accepted_job_ids": []
                    },
                    {
                        "filled": false,
                        "role_id": 2,
                        "filled_job_id": null,
                        "accepted_job_ids": [24, 28]
                    }
                ]
            }
        })
    );
}

#[test]
fn error_envelope_is_consistent() {
    let error =
        ErrorEnvelope::invalid_query("category_id", "category_id must be an unsigned integer");

    assert_eq!(
        serde_json::to_value(&error).unwrap(),
        json!({
            "error": {
                "code": "invalid_query",
                "message": "category_id must be an unsigned integer",
                "details": {
                    "field": "category_id"
                }
            }
        })
    );
}

#[test]
fn docs_include_minimal_contract_examples() {
    let readme = include_str!("../../README.md");
    let api_v2_doc = include_str!("../../docs/api-v2.md");
    let normalized_api_v2_doc = strip_whitespace(api_v2_doc);

    let collection = CollectionEnvelope {
        data: vec![sample_summary()],
        pagination: Pagination {
            total: 1,
            page: 1,
            per_page: 20,
            total_pages: 1,
        },
    };
    let detail = ListingMemberResponse {
        data: sample_detail(),
    };

    let collection_json = serde_json::to_string_pretty(&collection).unwrap();
    let detail_json = serde_json::to_string_pretty(&detail).unwrap();

    assert!(
        readme.contains("See [`docs/api-v2.md`](docs/api-v2.md)"),
        "README should point readers to the detailed v2 contract doc"
    );
    assert!(api_v2_doc.contains("`GET /api/v2/listings`"));
    assert!(api_v2_doc.contains("`GET /api/v2/listings/{id}`"));

    assert!(
        normalized_api_v2_doc.contains(&strip_whitespace(&collection_json)),
        "docs/api-v2.md collection example drifted from contract"
    );
    assert!(
        normalized_api_v2_doc.contains(&strip_whitespace(&detail_json)),
        "docs/api-v2.md detail example drifted from contract"
    );
}

fn strip_whitespace(value: &str) -> String {
    value.chars().filter(|ch| !ch.is_whitespace()).collect()
}

#[test]
fn id_inventory_is_stable() {
    assert_eq!(id_inventory::world_ids().len(), 131);
    assert_eq!(id_inventory::world_ids(), {
        let mut ids = crate::ffxiv::WORLDS.keys().copied().collect::<Vec<_>>();
        ids.sort_unstable();
        ids
    });
    assert_eq!(id_inventory::world_id(1167), 1167);

    assert_eq!(
        id_inventory::CATEGORY_IDS,
        [0, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768]
    );
    assert_eq!(id_inventory::category_id(DutyCategory::HighEndDuty), 64);
    assert_eq!(id_inventory::duty_id(1234), 1234);

    assert_eq!(id_inventory::DUTY_TYPE_IDS, [0, 1, 2]);
    assert_eq!(id_inventory::duty_type_id(DutyType::Other), 0);
    assert_eq!(id_inventory::duty_type_id(DutyType::Roulette), 1);
    assert_eq!(id_inventory::duty_type_id(DutyType::Normal), 2);

    assert_eq!(
        id_inventory::job_ids(),
        vec![
            1, 2, 3, 4, 5, 6, 7, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34,
            35, 36, 37, 38, 39, 40, 41, 42,
        ]
    );
    assert_eq!(id_inventory::filled_job_id(19), Some(19));
    assert_eq!(id_inventory::filled_job_id(8), Some(8));
    assert_eq!(
        id_inventory::accepted_job_ids(JobFlags::WHITE_MAGE | JobFlags::SCHOLAR),
        vec![24, 28]
    );

    assert_eq!(id_inventory::ROLE_IDS, [1, 2, 3]);
    assert_eq!(id_inventory::role_id(ffxiv_types_cn::Role::Dps), 1);
    assert_eq!(id_inventory::role_id(ffxiv_types_cn::Role::Healer), 2);
    assert_eq!(id_inventory::role_id(ffxiv_types_cn::Role::Tank), 3);
    assert_eq!(id_inventory::role_id_for_job_id(19), Some(3));
    assert_eq!(id_inventory::role_id_for_job_id(24), Some(2));
    assert_eq!(id_inventory::role_id_for_job_ids(&[24, 28]), Some(2));
    assert_eq!(id_inventory::role_id_for_job_ids(&[19, 24]), None);

    assert_eq!(id_inventory::OBJECTIVE_IDS, [1, 2, 4]);
    assert_eq!(
        id_inventory::objective_ids(
            ObjectiveFlags::DUTY_COMPLETION | ObjectiveFlags::PRACTICE | ObjectiveFlags::LOOT,
        ),
        vec![1, 2, 4]
    );
    assert_eq!(
        id_inventory::objective_ids(ObjectiveFlags::NONE),
        Vec::<u32>::new()
    );

    assert_eq!(id_inventory::CONDITION_IDS, [2, 4, 8]);
    assert_eq!(
        id_inventory::condition_ids(ConditionFlags::NONE),
        Vec::<u32>::new()
    );
    assert_eq!(
        id_inventory::condition_ids(
            ConditionFlags::DUTY_COMPLETE | ConditionFlags::DUTY_COMPLETE_WEEKLY_REWARD_UNCLAIMED,
        ),
        vec![2, 8]
    );

    assert_eq!(id_inventory::LOOT_RULE_IDS, [0, 1, 2, 3]);
    assert_eq!(id_inventory::loot_rule_id(LootRuleFlags::NONE), 0);
    assert_eq!(
        id_inventory::loot_rule_id(LootRuleFlags::GREED_ONLY | LootRuleFlags::LOOTMASTER),
        3
    );
}

#[test]
fn datacenter_not_exposed_in_primary_resources() {
    assert!(!id_inventory::PHASE_ONE_PRIMARY_EXPOSES_DATACENTER);

    for payload in [
        serde_json::to_value(sample_summary()).unwrap(),
        serde_json::to_value(sample_detail()).unwrap(),
    ] {
        let object = payload.as_object().unwrap();
        assert!(!object.contains_key("datacenter"));
        assert!(!object.contains_key("datacenter_id"));
    }
}

#[tokio::test]
async fn malformed_filters_return_400() {
    let cases = [
        (
            "/api/v2/listings?category_id=abc",
            ErrorEnvelope::invalid_query("category_id", "category_id must be an unsigned integer"),
        ),
        (
            "/api/v2/listings?job_ids=1,tank",
            ErrorEnvelope::invalid_query(
                "job_ids",
                "job_ids must be a comma-separated list of unsigned integers",
            ),
        ),
        (
            "/api/v2/listings?per_page=101",
            ErrorEnvelope::invalid_query("per_page", "per_page must be between 1 and 100"),
        ),
        (
            "/api/v2/listings?page=0",
            ErrorEnvelope::invalid_query("page", "page must be a positive integer"),
        ),
        (
            "/api/v2/listings?job_ids=",
            ErrorEnvelope::invalid_query(
                "job_ids",
                "job_ids must be a comma-separated list of unsigned integers",
            ),
        ),
        (
            "/api/v2/listings?unknown=1",
            ErrorEnvelope::invalid_query("unknown", "unknown is not a supported query parameter"),
        ),
    ];

    for (path, expected_error) in cases {
        let response = warp::test::request()
            .method("GET")
            .path(path)
            .reply(&crate::web::v2::listings::collection_route_for_tests())
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST, "path: {path}");
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(response.body()).unwrap(),
            serde_json::to_value(expected_error).unwrap(),
            "path: {path}",
        );
    }
}

#[tokio::test]
async fn legacy_label_filters_are_rejected() {
    let cases = [
        (
            "/api/v2/listings?world=%E7%8C%AB%E5%B0%8F%E8%83%96",
            ErrorEnvelope::invalid_query(
                "world",
                "world is not supported in v2; use created_world_id or home_world_id",
            ),
        ),
        (
            "/api/v2/listings?category=HighEndDuty",
            ErrorEnvelope::invalid_query(
                "category",
                "category is not supported in v2; use category_id",
            ),
        ),
    ];

    for (path, expected_error) in cases {
        let response = warp::test::request()
            .method("GET")
            .path(path)
            .reply(&crate::web::v2::listings::collection_route_for_tests())
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST, "path: {path}");
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(response.body()).unwrap(),
            serde_json::to_value(expected_error).unwrap(),
            "path: {path}",
        );
    }
}

#[tokio::test]
async fn lookup_routes_are_absent_from_the_full_router() {
    let lookup_paths = [
        "/api/v2/lookups",
        "/api/v2/lookups/",
        "/api/v2/lookups/worlds",
        "/api/v2/lookups/worlds/1167",
        "/api/v2/lookups/duties",
        "/api/v2/lookups/duties/55",
        "/api/v2/lookups/jobs",
        "/api/v2/lookups/categories",
    ];
    let router = crate::web::router(crate::web::state_for_router_tests().await);

    for path in lookup_paths {
        let response = warp::test::request()
            .method("GET")
            .path(path)
            .reply(&router)
            .await;

        assert_eq!(
            response.status(),
            StatusCode::METHOD_NOT_ALLOWED,
            "path {path} should be rejected consistently by the full router"
        );
    }
}

fn sample_summary() -> ListingSummary {
    ListingSummary {
        id: 900001,
        player_name: "Alice".into(),
        description: "Need clear".into(),
        created_world_id: 1167,
        home_world_id: 1167,
        category_id: 64,
        duty_id: 1234,
        duty_type_id: 0,
        min_item_level: 710,
        slots_filled: 6,
        slots_available: 8,
        time_left_seconds: 1200,
        updated_at: "2026-04-23T12:34:56Z".into(),
        is_cross_world: true,
        beginners_welcome: false,
    }
}

fn sample_detail() -> ListingDetail {
    ListingDetail {
        id: 900001,
        player_name: "Alice".into(),
        description: "Need clear".into(),
        created_world_id: 1167,
        home_world_id: 1167,
        category_id: 64,
        duty_id: 1234,
        duty_type_id: 0,
        min_item_level: 710,
        slots_filled: 6,
        slots_available: 8,
        time_left_seconds: 1200,
        updated_at: "2026-04-23T12:34:56Z".into(),
        is_cross_world: true,
        beginners_welcome: false,
        objective_ids: vec![1, 4],
        condition_ids: vec![2],
        loot_rule_id: 3,
        slots: vec![
            ListingSlot {
                filled: true,
                role_id: 3,
                filled_job_id: Some(19),
                accepted_job_ids: vec![],
            },
            ListingSlot {
                filled: false,
                role_id: 2,
                filled_job_id: None,
                accepted_job_ids: vec![24, 28],
            },
        ],
    }
}

#[tokio::test]
async fn duplicate_listing_id_returns_latest_active_detail() {
    let now = Utc::now();
    let active = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(2), 1200.0);
    let duplicate_old = queried_fixture(
        DUPLICATE_OLD_FIXTURE_JSON,
        now - Duration::seconds(30),
        900.0,
    );
    let expired = queried_fixture(EXPIRED_FIXTURE_JSON, now - Duration::minutes(1), -1.0);
    let expected_updated_at = duplicate_old.updated_at.to_rfc3339();

    let response = warp::test::request()
        .method("GET")
        .path("/api/v2/listings/12345")
        .reply(&member_route_for_tests(vec![expired, duplicate_old, active]))
        .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = serde_json::from_slice::<serde_json::Value>(response.body()).unwrap();
    assert_eq!(body["data"]["id"], json!(12345));
    assert_eq!(body["data"]["player_name"], json!("Duplicate"));
    assert_eq!(body["data"]["time_left_seconds"], json!(900));
    assert_eq!(body["data"]["updated_at"], json!(expected_updated_at));
}

#[test]
fn listings_projection_excludes_labels() {
    let now = Utc::now();
    let active = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.9);
    let summary = project_listing_summary(&active).expect("active listing should project");
    let detail = project_listing_detail(&active).expect("active detail should project");

    assert_eq!(
        summary,
        ListingSummary {
            id: 12345,
            player_name: "Active".into(),
            description: "Active fixture".into(),
            created_world_id: 73,
            home_world_id: 73,
            category_id: 64,
            duty_id: 55,
            duty_type_id: 2,
            min_item_level: 0,
            slots_filled: 1,
            slots_available: 8,
            time_left_seconds: 1200,
            updated_at: active.updated_at.to_rfc3339(),
            is_cross_world: true,
            beginners_welcome: false,
        }
    );

    assert_eq!(
        detail,
        ListingDetail {
            id: 12345,
            player_name: "Active".into(),
            description: "Active fixture".into(),
            created_world_id: 73,
            home_world_id: 73,
            category_id: 64,
            duty_id: 55,
            duty_type_id: 2,
            min_item_level: 0,
            slots_filled: 1,
            slots_available: 8,
            time_left_seconds: 1200,
            updated_at: active.updated_at.to_rfc3339(),
            is_cross_world: true,
            beginners_welcome: false,
            objective_ids: vec![1, 2],
            condition_ids: vec![],
            loot_rule_id: 0,
            slots: vec![ListingSlot {
                filled: true,
                role_id: 1,
                filled_job_id: Some(5),
                accepted_job_ids: vec![],
            }],
        }
    );

    for payload in [
        serde_json::to_value(&summary).unwrap(),
        serde_json::to_value(&detail).unwrap(),
    ] {
        let object = payload
            .as_object()
            .expect("listing projections serialize as objects");
        for forbidden in [
            "name",
            "created_world",
            "home_world",
            "category",
            "duty",
            "duty_type",
            "datacenter",
        ] {
            assert!(
                !object.contains_key(forbidden),
                "projection unexpectedly exposed label field {forbidden}"
            );
        }
    }
}

#[tokio::test]
async fn listing_detail_is_ids_only() {
    let now = Utc::now();
    let active = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.9);
    let expected_updated_at = active.updated_at.to_rfc3339();

    let response = warp::test::request()
        .method("GET")
        .path("/api/v2/listings/12345")
        .reply(&member_route_for_tests(vec![active]))
        .await;

    assert_eq!(response.status(), StatusCode::OK);

    let body = serde_json::from_slice::<serde_json::Value>(response.body()).unwrap();
    let object = body["data"].as_object().cloned().unwrap();

    assert_eq!(
        body["data"],
        json!({
            "id": 12345,
            "player_name": "Active",
            "description": "Active fixture",
            "created_world_id": 73,
            "home_world_id": 73,
            "category_id": 64,
            "duty_id": 55,
            "duty_type_id": 2,
            "min_item_level": 0,
            "slots_filled": 1,
            "slots_available": 8,
            "time_left_seconds": 1200,
            "updated_at": expected_updated_at,
            "is_cross_world": true,
            "beginners_welcome": false,
            "objective_ids": [1, 2],
            "condition_ids": [],
            "loot_rule_id": 0,
            "slots": [
                {
                    "filled": true,
                    "role_id": 1,
                    "filled_job_id": 5,
                    "accepted_job_ids": [],
                }
            ],
        })
    );

    for forbidden in [
        "name",
        "created_world",
        "home_world",
        "category",
        "duty",
        "duty_type",
        "objective",
        "conditions",
        "loot_rules",
        "datacenter",
        "role",
        "job",
        "job_id",
    ] {
        assert!(
            !object.contains_key(forbidden),
            "detail unexpectedly exposed {forbidden}"
        );
    }
}

#[test]
fn listings_summary_is_ids_only() {
    let now = Utc::now();
    let active = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);
    let cross_world = queried_fixture(CROSS_WORLD_FIXTURE_JSON, now - Duration::minutes(2), 900.0);
    let response = collection_response_from_documents(
        ListingsQuery {
            page: 1,
            per_page: 20,
            search: Some("fixture".into()),
            ..Default::default()
        },
        [&active, &cross_world],
    );

    assert_eq!(response.pagination.total, 2);
    assert_eq!(response.pagination.page, 1);
    assert_eq!(response.pagination.per_page, 20);
    assert_eq!(response.pagination.total_pages, 1);
    assert_eq!(response.data.len(), 2);

    assert_eq!(
        serde_json::to_value(&response.data).unwrap(),
        json!([
            {
                "id": 12345,
                "player_name": "Active",
                "description": "Active fixture",
                "created_world_id": 73,
                "home_world_id": 73,
                "category_id": 64,
                "duty_id": 55,
                "duty_type_id": 2,
                "min_item_level": 0,
                "slots_filled": 1,
                "slots_available": 8,
                "time_left_seconds": 1200,
                "updated_at": active.updated_at.to_rfc3339(),
                "is_cross_world": true,
                "beginners_welcome": false,
            },
            {
                "id": 12346,
                "player_name": "Cross World",
                "description": "Cross-world fixture",
                "created_world_id": 73,
                "home_world_id": 73,
                "category_id": 64,
                "duty_id": 55,
                "duty_type_id": 2,
                "min_item_level": 0,
                "slots_filled": 1,
                "slots_available": 8,
                "time_left_seconds": 900,
                "updated_at": cross_world.updated_at.to_rfc3339(),
                "is_cross_world": true,
                "beginners_welcome": false,
            }
        ])
    );
}

#[test]
fn collection_response_filters_public_job_ids_by_inventory_mapping() {
    let now = Utc::now();
    let mut white_mage = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);
    white_mage.listing.slots[0].accepting = JobFlags::WHITE_MAGE;

    let mut red_mage = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);
    red_mage.listing.id = 54321;
    red_mage.listing.slots[0].accepting = JobFlags::RED_MAGE;

    let response = collection_response_from_documents(
        ListingsQuery {
            job_ids: vec![24],
            ..Default::default()
        },
        [&white_mage, &red_mage],
    );

    assert_eq!(response.pagination.total, 1);
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].id, 12345);
    assert_eq!(response.data[0].player_name, "Active");
}

#[test]
fn duplicate_job_ids_do_not_change_matching_results() {
    let now = Utc::now();
    let mut white_mage = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);
    white_mage.listing.slots[0].accepting = JobFlags::WHITE_MAGE;

    let mut red_mage = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);
    red_mage.listing.id = 54321;
    red_mage.listing.slots[0].accepting = JobFlags::RED_MAGE;

    let response = collection_response_from_documents(
        ListingsQuery {
            job_ids: vec![24, 24],
            ..Default::default()
        },
        [&white_mage, &red_mage],
    );

    assert_eq!(response.pagination.total, 1);
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].id, 12345);
    assert_eq!(response.data[0].player_name, "Active");
}

#[test]
fn scalar_filters_match_expected_documents() {
    let now = Utc::now();
    let mut created_world_match = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);
    created_world_match.listing.created_world = 1167;

    let mut home_world_match = queried_fixture(CROSS_WORLD_FIXTURE_JSON, now - Duration::minutes(1), 900.0);
    home_world_match.listing.id = 54321;
    home_world_match.listing.home_world = 1174;

    let mut category_and_duty_match = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 600.0);
    category_and_duty_match.listing.id = 67890;
    category_and_duty_match.listing.category = DutyCategory::Raid;
    category_and_duty_match.listing.duty = 777;
    let category_id = id_inventory::category_id(category_and_duty_match.listing.category);

    let documents = [&created_world_match, &home_world_match, &category_and_duty_match];

    let created_world_response = collection_response_from_documents(
        ListingsQuery {
            created_world_id: Some(1167),
            ..Default::default()
        },
        documents,
    );
    assert_eq!(created_world_response.pagination.total, 1);
    assert_eq!(created_world_response.data[0].id, 12345);

    let home_world_response = collection_response_from_documents(
        ListingsQuery {
            home_world_id: Some(1174),
            ..Default::default()
        },
        documents,
    );
    assert_eq!(home_world_response.pagination.total, 1);
    assert_eq!(home_world_response.data[0].id, 54321);

    let category_response = collection_response_from_documents(
        ListingsQuery {
            category_id: Some(category_id),
            ..Default::default()
        },
        documents,
    );
    assert_eq!(category_response.pagination.total, 1);
    assert_eq!(category_response.data[0].id, 67890);

    let duty_response = collection_response_from_documents(
        ListingsQuery {
            duty_id: Some(777),
            ..Default::default()
        },
        documents,
    );
    assert_eq!(duty_response.pagination.total, 1);
    assert_eq!(duty_response.data[0].id, 67890);
}

#[tokio::test]
async fn collection_decode_failure_returns_internal_error() {
    let response = collection_response_from_raw_documents_for_tests(
        ListingsQuery::default(),
        vec![doc! {
            "listing": { "id": 12345 },
            "updated_at": chrono::Utc::now(),
        }],
    );
    let status = response.status();
    let body = warp::hyper::body::to_bytes(response.into_body()).await.unwrap();

    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&body).unwrap(),
        json!({
            "error": {
                "code": "internal_error",
                "message": "Failed to load API v2 listings",
                "details": {}
            }
        })
    );
}

#[tokio::test]
async fn well_formed_unknown_filters_return_empty_collection() {
    let cases = [
        "/api/v2/listings?created_world_id=999999",
        "/api/v2/listings?home_world_id=999999",
        "/api/v2/listings?category_id=999999",
        "/api/v2/listings?duty_id=999999",
        "/api/v2/listings?job_ids=999999",
        "/api/v2/listings?search=fixture&created_world_id=999999",
    ];

    for path in cases {
        let response = warp::test::request()
            .method("GET")
            .path(path)
            .reply(&crate::web::v2::listings::collection_route_for_tests())
            .await;

        assert_eq!(response.status(), StatusCode::OK, "path: {path}");
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(response.body()).unwrap(),
            json!({
                "data": [],
                "pagination": {
                    "total": 0,
                    "page": 1,
                    "per_page": 20,
                    "total_pages": 0,
                }
            }),
            "path: {path}",
        );
    }
}

#[tokio::test]
async fn per_page_hundred_is_accepted() {
    let response = warp::test::request()
        .method("GET")
        .path("/api/v2/listings?per_page=100")
        .reply(&crate::web::v2::listings::collection_route_for_tests())
        .await;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(response.body()).unwrap(),
        json!({
            "data": [],
            "pagination": {
                "total": 0,
                "page": 1,
                "per_page": 100,
                "total_pages": 0,
            }
        })
    );
}

#[test]
fn expired_listings_are_excluded_from_v2() {
    let now = Utc::now();
    let active = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);
    let expired = queried_fixture(EXPIRED_FIXTURE_JSON, now - Duration::minutes(1), -1.0);

    assert!(project_listing_summary(&expired).is_none());
    assert_eq!(
        project_listing_summaries([&expired]),
        Vec::<ListingSummary>::new()
    );
    assert_eq!(
        project_listing_summaries([&expired, &active]),
        vec![project_listing_summary(&active).unwrap()]
    );
    assert!(resolve_listing_detail(12345, [&expired]).is_none());
}

#[test]
fn active_window_keeps_just_fresh_listings_and_drops_stale_ones() {
    let now = Utc::now();
    let just_fresh = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(4) - Duration::seconds(59), 1200.0);
    let stale = queried_fixture(CROSS_WORLD_FIXTURE_JSON, now - Duration::minutes(5) - Duration::seconds(1), 1200.0);

    let projected = project_listing_summaries([&just_fresh, &stale]);

    assert_eq!(projected.len(), 1);
    assert_eq!(projected[0].id, 12345);
    assert_eq!(projected[0].player_name, "Active");
}

#[test]
fn detail_projection_prefers_latest_active_duplicate_id() {
    let now = Utc::now();
    let active = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(2), 1200.0);
    let duplicate_old = queried_fixture(
        DUPLICATE_OLD_FIXTURE_JSON,
        now - Duration::seconds(30),
        900.0,
    );
    let expired = queried_fixture(EXPIRED_FIXTURE_JSON, now - Duration::minutes(1), -1.0);

    let detail = resolve_listing_detail(12345, [&expired, &duplicate_old, &active])
        .expect("latest active duplicate should resolve");

    assert_eq!(detail.player_name, "Duplicate");
    assert_eq!(detail.time_left_seconds, 900);
    assert_eq!(detail.updated_at, duplicate_old.updated_at.to_rfc3339());
}

#[tokio::test]
async fn expired_or_missing_listing_returns_404() {
    let now = Utc::now();
    let expired = queried_fixture(EXPIRED_FIXTURE_JSON, now - Duration::minutes(1), -1.0);
    let active = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);
    let private = private_queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);
    let private_expired = private_queried_fixture(EXPIRED_FIXTURE_JSON, now - Duration::minutes(1), -1.0);

    let expired_response = warp::test::request()
        .method("GET")
        .path("/api/v2/listings/12345")
        .reply(&member_route_for_tests(vec![expired]))
        .await;
    assert_eq!(expired_response.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(expired_response.body()).unwrap(),
        json!({
            "error": {
                "code": "not_found",
                "message": "Listing not found",
                "details": {
                    "id": 12345,
                }
            }
        })
    );

    let missing_response = warp::test::request()
        .method("GET")
        .path("/api/v2/listings/99999")
        .reply(&member_route_for_tests(vec![active]))
        .await;
    assert_eq!(missing_response.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(missing_response.body()).unwrap(),
        json!({
            "error": {
                "code": "not_found",
                "message": "Listing not found",
                "details": {
                    "id": 99999,
                }
            }
        })
    );

    let private_response = warp::test::request()
        .method("GET")
        .path("/api/v2/listings/12345")
        .reply(&member_route_for_tests(vec![private]))
        .await;
    assert_eq!(private_response.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(private_response.body()).unwrap(),
        json!({
            "error": {
                "code": "not_found",
                "message": "Listing not found",
                "details": {
                    "id": 12345,
                }
            }
        })
    );

    let private_expired_response = warp::test::request()
        .method("GET")
        .path("/api/v2/listings/12345")
        .reply(&member_route_for_tests(vec![private_expired]))
        .await;
    assert_eq!(private_expired_response.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(private_expired_response.body()).unwrap(),
        json!({
            "error": {
                "code": "not_found",
                "message": "Listing not found",
                "details": {
                    "id": 12345,
                }
            }
        })
    );
}

fn queried_fixture(
    fixture_json: &str,
    updated_at: chrono::DateTime<Utc>,
    time_left: f64,
) -> QueriedListing {
    let listing = serde_json::from_str(fixture_json).expect("fixture must parse");

    QueriedListing {
        created_at: updated_at - Duration::minutes(1),
        updated_at,
        updated_minute: updated_at,
        time_left,
        listing,
    }
}

fn private_queried_fixture(
    fixture_json: &str,
    updated_at: chrono::DateTime<Utc>,
    time_left: f64,
) -> QueriedListing {
    let mut fixture = queried_fixture(fixture_json, updated_at, time_left);
    fixture.listing.search_area = crate::listing::SearchAreaFlags::PRIVATE;
    fixture
}

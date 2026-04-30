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
    assert!(api_v2_doc.contains("`GET /api/v2/listings?datacenter=Aether,Primal`"));
    assert!(api_v2_doc.contains("`GET /api/v2/listings?region=North-America,Japan`"));

    assert!(
        normalized_api_v2_doc.contains(&strip_whitespace(&collection_json)),
        "docs/api-v2.md collection example drifted from contract"
    );
    assert!(
        normalized_api_v2_doc.contains(&strip_whitespace(&detail_json)),
        "docs/api-v2.md detail example drifted from contract"
    );
}

#[test]
fn docs_examples_match_v2_contract() {
    docs_include_minimal_contract_examples();
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
        (
            "/api/v2/listings?created_world_id=",
            ErrorEnvelope::invalid_query(
                "created_world_id",
                "created_world_id must be a comma-separated list of unsigned integers",
            ),
        ),
        (
            "/api/v2/listings?home_world_id=",
            ErrorEnvelope::invalid_query(
                "home_world_id",
                "home_world_id must be a comma-separated list of unsigned integers",
            ),
        ),
        (
            "/api/v2/listings?created_world_id=73,",
            ErrorEnvelope::invalid_query(
                "created_world_id",
                "created_world_id must be a comma-separated list of unsigned integers",
            ),
        ),
        (
            "/api/v2/listings?home_world_id=73,abc",
            ErrorEnvelope::invalid_query(
                "home_world_id",
                "home_world_id must be a comma-separated list of unsigned integers",
            ),
        ),
        (
            "/api/v2/listings?datacenter=",
            ErrorEnvelope::invalid_query(
                "datacenter",
                "datacenter must be a comma-separated list of names",
            ),
        ),
        (
            "/api/v2/listings?region=",
            ErrorEnvelope::invalid_query(
                "region",
                "region must be a comma-separated list of names",
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
fn listings_projection_excludes_legacy_labels() {
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
        "datacenter",
        "region",
        "world",
        "datacenter_id",
        "region_id",
        "name",
        "created_world",
        "home_world",
        "category",
        "duty",
        "duty_type",
        "objective",
        "conditions",
        "loot_rules",
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

    for payload in serde_json::to_value(&response.data)
        .unwrap()
        .as_array()
        .unwrap()
    {
        let object = payload.as_object().unwrap();
        for forbidden in ["datacenter", "region", "world", "datacenter_id", "region_id"] {
            assert!(
                !object.contains_key(forbidden),
                "summary unexpectedly exposed {forbidden}"
            );
        }
    }
}

#[test]
fn listing_summary_omits_datacenter_and_region() {
    let now = Utc::now();
    let active = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);
    let summary = project_listing_summary(&active).expect("active listing should project");
    let object = serde_json::to_value(&summary).unwrap();
    let object = object.as_object().unwrap();

    assert!(object.contains_key("created_world_id"));
    assert!(object.contains_key("home_world_id"));
    assert!(!object.contains_key("datacenter"));
    assert!(!object.contains_key("region"));
}

#[test]
fn listing_detail_omits_datacenter_and_region() {
    let now = Utc::now();
    let active = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);
    let detail = project_listing_detail(&active).expect("active detail should project");
    let object = serde_json::to_value(&detail).unwrap();
    let object = object.as_object().unwrap();

    assert!(object.contains_key("created_world_id"));
    assert!(object.contains_key("home_world_id"));
    assert!(!object.contains_key("datacenter"));
    assert!(!object.contains_key("region"));
}

#[test]
fn listings_do_not_expose_world_name_or_lookup_ids() {
    let now = Utc::now();
    let active = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);
    let summary = project_listing_summary(&active).expect("active listing should project");
    let detail = project_listing_detail(&active).expect("active detail should project");

    for payload in [
        serde_json::to_value(&summary).unwrap(),
        serde_json::to_value(&detail).unwrap(),
    ] {
        let object = payload.as_object().unwrap();
        for forbidden in ["world", "datacenter_id", "region_id"] {
            assert!(
                !object.contains_key(forbidden),
                "payload unexpectedly exposed {forbidden}"
            );
        }
    }
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
            created_world_id: vec![1167],
            ..Default::default()
        },
        documents,
    );
    assert_eq!(created_world_response.pagination.total, 1);
    assert_eq!(created_world_response.data[0].id, 12345);

    let home_world_response = collection_response_from_documents(
        ListingsQuery {
            home_world_id: vec![1174],
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

#[test]
fn created_world_id_supports_comma_separated_or() {
    let now = Utc::now();
    let created_world_73 = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);

    let mut created_world_1167 = queried_fixture(CROSS_WORLD_FIXTURE_JSON, now - Duration::minutes(1), 900.0);
    created_world_1167.listing.id = 54321;
    created_world_1167.listing.created_world = 1167;

    let mut created_world_1174 = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 600.0);
    created_world_1174.listing.id = 67890;
    created_world_1174.listing.created_world = 1174;

    let response = collection_response_from_documents(
        ListingsQuery {
            created_world_id: vec![73, 1167],
            ..Default::default()
        },
        [&created_world_73, &created_world_1167, &created_world_1174],
    );

    assert_eq!(response.pagination.total, 2);
    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0].id, 12345);
    assert_eq!(response.data[1].id, 54321);
}

#[test]
fn home_world_id_supports_comma_separated_or() {
    let now = Utc::now();
    let home_world_73 = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);

    let mut home_world_1174 = queried_fixture(CROSS_WORLD_FIXTURE_JSON, now - Duration::minutes(1), 900.0);
    home_world_1174.listing.id = 54321;
    home_world_1174.listing.home_world = 1174;

    let mut home_world_1167 = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 600.0);
    home_world_1167.listing.id = 67890;
    home_world_1167.listing.home_world = 1167;

    let response = collection_response_from_documents(
        ListingsQuery {
            home_world_id: vec![73, 1174],
            ..Default::default()
        },
        [&home_world_73, &home_world_1174, &home_world_1167],
    );

    assert_eq!(response.pagination.total, 2);
    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0].id, 12345);
    assert_eq!(response.data[1].id, 54321);
}

#[test]
fn created_and_home_world_filters_intersect_across_fields() {
    let now = Utc::now();

    let mut intersection = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);
    intersection.listing.created_world = 73;
    intersection.listing.home_world = 1174;

    let mut created_only = queried_fixture(CROSS_WORLD_FIXTURE_JSON, now - Duration::minutes(1), 900.0);
    created_only.listing.id = 54321;
    created_only.listing.created_world = 73;
    created_only.listing.home_world = 1167;

    let mut home_only = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 600.0);
    home_only.listing.id = 67890;
    home_only.listing.created_world = 1167;
    home_only.listing.home_world = 1174;

    let response = collection_response_from_documents(
        ListingsQuery {
            created_world_id: vec![73, 1167],
            home_world_id: vec![1174],
            ..Default::default()
        },
        [&intersection, &created_only, &home_only],
    );

    assert_eq!(response.pagination.total, 2);
    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0].id, 12345);
    assert_eq!(response.data[1].id, 67890);
}

#[test]
fn duplicate_world_ids_do_not_change_results() {
    let now = Utc::now();
    let created_world_73 = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);

    let mut created_world_1167 = queried_fixture(CROSS_WORLD_FIXTURE_JSON, now - Duration::minutes(1), 900.0);
    created_world_1167.listing.id = 54321;
    created_world_1167.listing.created_world = 1167;

    let response = collection_response_from_documents(
        ListingsQuery {
            created_world_id: vec![73, 73],
            ..Default::default()
        },
        [&created_world_73, &created_world_1167],
    );

    assert_eq!(response.pagination.total, 1);
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].id, 12345);
}

#[test]
fn mixed_validity_world_id_lists_ignore_unknown_members() {
    let now = Utc::now();
    let created_world_73 = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);

    let mut created_world_1167 = queried_fixture(CROSS_WORLD_FIXTURE_JSON, now - Duration::minutes(1), 900.0);
    created_world_1167.listing.id = 54321;
    created_world_1167.listing.created_world = 1167;

    let response = collection_response_from_documents(
        ListingsQuery {
            created_world_id: vec![73, 999999],
            ..Default::default()
        },
        [&created_world_73, &created_world_1167],
    );

    assert_eq!(response.pagination.total, 1);
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].id, 12345);
}

#[test]
fn datacenter_supports_comma_separated_or() {
    let now = Utc::now();
    let first = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);

    let mut second = queried_fixture(CROSS_WORLD_FIXTURE_JSON, now - Duration::minutes(1), 900.0);
    second.listing.id = 54321;
    second.listing.created_world = 1167;

    let first_datacenter = first.listing.data_centre_name().unwrap().to_string();
    let second_datacenter = second.listing.data_centre_name().unwrap().to_string();

    let response = collection_response_from_documents(
        ListingsQuery {
            datacenter: Some(format!("{first_datacenter},{second_datacenter}")),
            ..Default::default()
        },
        [&first, &second],
    );

    assert_eq!(response.pagination.total, 2);
    assert_eq!(response.data.len(), 2);
}

#[test]
fn region_supports_comma_separated_or() {
    let now = Utc::now();
    let first = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);

    let mut second = queried_fixture(CROSS_WORLD_FIXTURE_JSON, now - Duration::minutes(1), 900.0);
    second.listing.id = 54321;
    second.listing.created_world = 1167;

    let first_region = if first.listing.data_centre_name() == Some("Aether") {
        "North-America"
    } else {
        "中国"
    }
    .to_string();
    let second_region = if second.listing.data_centre_name() == Some("Aether") {
        "North-America"
    } else {
        "中国"
    }
    .to_string();

    let response = collection_response_from_documents(
        ListingsQuery {
            region: Some(format!("{first_region},{second_region}")),
            ..Default::default()
        },
        [&first, &second],
    );

    assert_eq!(response.pagination.total, 2);
    assert_eq!(response.data.len(), 2);
}

#[tokio::test]
async fn unknown_datacenter_names_return_empty_collection() {
    let response = warp::test::request()
        .method("GET")
        .path("/api/v2/listings?datacenter=NoSuchDatacenter")
        .reply(&crate::web::v2::listings::collection_route_for_tests())
        .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body = serde_json::from_slice::<serde_json::Value>(response.body()).unwrap();
    assert_eq!(body["pagination"]["total"], json!(0));
}

#[tokio::test]
async fn unknown_region_names_return_empty_collection() {
    let response = warp::test::request()
        .method("GET")
        .path("/api/v2/listings?region=NoSuchRegion")
        .reply(&crate::web::v2::listings::collection_route_for_tests())
        .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body = serde_json::from_slice::<serde_json::Value>(response.body()).unwrap();
    assert_eq!(body["pagination"]["total"], json!(0));
}

#[test]
fn mixed_validity_datacenter_and_region_lists_use_valid_subset() {
    let now = Utc::now();
    let first = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);

    let first_datacenter = first.listing.data_centre_name().unwrap().to_string();
    let first_region = if first.listing.data_centre_name() == Some("Aether") {
        "North-America"
    } else {
        "中国"
    }
    .to_string();

    let datacenter_response = collection_response_from_documents(
        ListingsQuery {
            datacenter: Some(format!("{first_datacenter},NoSuchDatacenter")),
            ..Default::default()
        },
        [&first],
    );
    assert_eq!(datacenter_response.pagination.total, 1);

    let region_response = collection_response_from_documents(
        ListingsQuery {
            region: Some(format!("{first_region},NoSuchRegion")),
            ..Default::default()
        },
        [&first],
    );
    assert_eq!(region_response.pagination.total, 1);
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

/// Thin acceptance wrapper — world= continues to be rejected.
#[tokio::test]
async fn world_query_remains_unsupported() {
    let response = warp::test::request()
        .method("GET")
        .path("/api/v2/listings?world=%E7%8C%AB%E5%B0%8F%E8%83%96")
        .reply(&crate::web::v2::listings::collection_route_for_tests())
        .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(response.body()).unwrap(),
        serde_json::to_value(ErrorEnvelope::invalid_query(
            "world",
            "world is not supported in v2; use created_world_id or home_world_id",
        ))
        .unwrap(),
    );
}

/// Thin acceptance wrapper for the empty / non-numeric segment cases.
#[tokio::test]
async fn created_world_id_rejects_empty_or_non_numeric_segments() {
    for path in [
        "/api/v2/listings?created_world_id=",
        "/api/v2/listings?created_world_id=73,",
        "/api/v2/listings?created_world_id=73,abc",
    ] {
        let response = warp::test::request()
            .method("GET")
            .path(path)
            .reply(&crate::web::v2::listings::collection_route_for_tests())
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST, "path: {path}");
    }
}

/// Thin acceptance wrapper for the empty / non-numeric segment cases.
#[tokio::test]
async fn home_world_id_rejects_empty_or_non_numeric_segments() {
    for path in [
        "/api/v2/listings?home_world_id=",
        "/api/v2/listings?home_world_id=73,",
        "/api/v2/listings?home_world_id=73,abc",
    ] {
        let response = warp::test::request()
            .method("GET")
            .path(path)
            .reply(&crate::web::v2::listings::collection_route_for_tests())
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST, "path: {path}");
    }
}

#[tokio::test]
async fn datacenter_rejects_empty_segments() {
    for path in ["/api/v2/listings?datacenter=", "/api/v2/listings?datacenter=Aether,"] {
        let response = warp::test::request()
            .method("GET")
            .path(path)
            .reply(&crate::web::v2::listings::collection_route_for_tests())
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST, "path: {path}");
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(response.body()).unwrap(),
            serde_json::to_value(ErrorEnvelope::invalid_query(
                "datacenter",
                "datacenter must be a comma-separated list of names",
            ))
            .unwrap(),
            "path: {path}",
        );
    }
}

#[tokio::test]
async fn region_rejects_empty_segments() {
    for path in ["/api/v2/listings?region=", "/api/v2/listings?region=Japan,"] {
        let response = warp::test::request()
            .method("GET")
            .path(path)
            .reply(&crate::web::v2::listings::collection_route_for_tests())
            .await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST, "path: {path}");
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(response.body()).unwrap(),
            serde_json::to_value(ErrorEnvelope::invalid_query(
                "region",
                "region must be a comma-separated list of names",
            ))
            .unwrap(),
            "path: {path}",
        );
    }
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

// Task 4: Precedence and validation-order behavior tests

#[test]
fn world_id_filters_mask_datacenter_and_region() {
    let now = Utc::now();
    let listing_in_datacenter = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);

    let datacenter_name = listing_in_datacenter.listing.data_centre_name().unwrap().to_string();
    let region_name = "North-America";

    // With only datacenter filter - should return the listing
    let datacenter_response = collection_response_from_documents(
        ListingsQuery {
            datacenter: Some(datacenter_name.clone()),
            ..Default::default()
        },
        [&listing_in_datacenter],
    );
    assert_eq!(datacenter_response.pagination.total, 1, "datacenter filter alone should match");

    // With only region filter - should return the listing
    let region_response = collection_response_from_documents(
        ListingsQuery {
            region: Some(region_name.to_string()),
            ..Default::default()
        },
        [&listing_in_datacenter],
    );
    assert_eq!(region_response.pagination.total, 1, "region filter alone should match");

    // With world-id + datacenter - datacenter should be masked, listing should NOT match
    let world_plus_dc_response = collection_response_from_documents(
        ListingsQuery {
            created_world_id: vec![999999], // Unknown world - no matches
            datacenter: Some(datacenter_name.clone()),
            ..Default::default()
        },
        [&listing_in_datacenter],
    );
    assert_eq!(
        world_plus_dc_response.pagination.total, 0,
        "world-id filter should mask datacenter"
    );

    // With world-id + region - region should be masked, listing should NOT match
    let world_plus_region_response = collection_response_from_documents(
        ListingsQuery {
            created_world_id: vec![999999],
            region: Some(region_name.to_string()),
            ..Default::default()
        },
        [&listing_in_datacenter],
    );
    assert_eq!(
        world_plus_region_response.pagination.total, 0,
        "world-id filter should mask region"
    );

    // With valid world-id + datacenter - datacenter should be masked, but world-id should match
    let valid_world_plus_dc_response = collection_response_from_documents(
        ListingsQuery {
            created_world_id: vec![73], // This listing's created_world
            datacenter: Some("NoSuchDatacenter".to_string()),
            ..Default::default()
        },
        [&listing_in_datacenter],
    );
    assert_eq!(
        valid_world_plus_dc_response.pagination.total, 1,
        "valid world-id should match even with invalid datacenter (masked)"
    );
}

#[test]
fn datacenter_masks_region() {
    let now = Utc::now();
    let listing_in_aether = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);

    // With only region filter - should return the listing
    let region_response = collection_response_from_documents(
        ListingsQuery {
            region: Some("North-America".to_string()),
            ..Default::default()
        },
        [&listing_in_aether],
    );
    assert_eq!(region_response.pagination.total, 1, "region filter alone should match");

    let dc_unknown_region_match_response = collection_response_from_documents(
        ListingsQuery {
            datacenter: Some("NoSuchDatacenter".to_string()),
            region: Some("North-America".to_string()),
            ..Default::default()
        },
        [&listing_in_aether],
    );
    assert_eq!(
        dc_unknown_region_match_response.pagination.total, 0,
        "unknown datacenter should still mask region and yield no matches"
    );

    // With valid datacenter + region - well-formed datacenter masks region
    // Datacenter matches, so listing should be returned (region is masked)
    let datacenter_name = listing_in_aether.listing.data_centre_name().unwrap().to_string();
    let dc_matches_region_response = collection_response_from_documents(
        ListingsQuery {
            datacenter: Some(datacenter_name),
            region: Some("Europe".to_string()), // Different region
            ..Default::default()
        },
        [&listing_in_aether],
    );
    assert_eq!(
        dc_matches_region_response.pagination.total, 1,
        "valid datacenter should match even when region doesn't (region is masked)"
    );
}

#[tokio::test]
async fn malformed_masked_filters_still_return_invalid_query() {
    // Validation happens first - malformed filters return 400 even if they would be masked
    let cases = [
        (
            "/api/v2/listings?created_world_id=73&datacenter=",
            "empty datacenter should return 400 even when world-id is valid",
        ),
        (
            "/api/v2/listings?created_world_id=73&region=",
            "empty region should return 400 even when world-id is valid",
        ),
        (
            "/api/v2/listings?home_world_id=73&datacenter=",
            "empty datacenter should return 400 even when home_world-id is valid",
        ),
        (
            "/api/v2/listings?datacenter=Gaia&region=",
            "empty region should return 400 even when datacenter is valid",
        ),
    ];

    for (path, description) in cases {
        let response = warp::test::request()
            .method("GET")
            .path(path)
            .reply(&crate::web::v2::listings::collection_route_for_tests())
            .await;

        assert_eq!(
            response.status(),
            StatusCode::BAD_REQUEST,
            "{description}: path {path}"
        );
    }
}

#[test]
fn pipeline_and_in_memory_precedence_semantics_match() {
    let now = Utc::now();
    let listing_73 = queried_fixture(ACTIVE_FIXTURE_JSON, now - Duration::minutes(1), 1200.0);

    let mut listing_1167 = queried_fixture(CROSS_WORLD_FIXTURE_JSON, now - Duration::minutes(1), 900.0);
    listing_1167.listing.id = 54321;
    listing_1167.listing.created_world = 1167;

    // Test 1: world-id masks datacenter and region
    let test_cases = [
        // (query, expected_ids)
        (
            ListingsQuery {
                created_world_id: vec![73],
                datacenter: Some("Gaia".to_string()),
                ..Default::default()
            },
            vec![12345], // Only listing 73 matches, datacenter masked
        ),
        (
            ListingsQuery {
                created_world_id: vec![73],
                region: Some("Europe".to_string()),
                ..Default::default()
            },
            vec![12345], // Only listing 73 matches, region masked
        ),
        // Test 2: datacenter masks region
        (
            ListingsQuery {
                datacenter: Some("Aether".to_string()),
                region: Some("Europe".to_string()),
                ..Default::default()
            },
            vec![12345], // Only listing 73 (Aether), region masked
        ),
        // Test 3: No precedence - all active
        (
            ListingsQuery {
                datacenter: Some("Aether".to_string()),
                ..Default::default()
            },
            vec![12345], // Listing 73 is in Aether
        ),
        (
            ListingsQuery {
                region: Some("North-America".to_string()),
                ..Default::default()
            },
            vec![12345], // Listing 73 is in North America (Aether)
        ),
    ];

    for (query, expected_ids) in test_cases {
        let response = collection_response_from_documents(
            query.clone(),
            [&listing_73, &listing_1167],
        );

        let actual_ids: Vec<u32> = response.data.iter().map(|l| l.id).collect();
        assert_eq!(
            actual_ids, expected_ids,
            "query: created_world_id={:?}, datacenter={:?}, region={:?}",
            query.created_world_id, query.datacenter, query.region
        );
    }
}

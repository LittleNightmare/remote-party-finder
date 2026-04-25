use std::{collections::HashMap, convert::Infallible, sync::Arc};

use chrono::{Duration, Utc};
use mongodb::bson::{doc, Document};
use serde_json::{Map, Value};
use tokio_stream::StreamExt;
use warp::{filters::BoxedFilter, http::StatusCode, reply::Response, Filter, Reply};

use crate::{
    listing::{PartyFinderListing, SearchAreaFlags},
    listing_container::QueriedListing,
    sestring_ext::SeStringExt,
    web::State,
};

use super::{
    contracts::{
        CollectionEnvelope, ErrorEnvelope, ListingCollectionResponse, ListingDetail,
        ListingMemberResponse, ListingSlot, ListingSummary, Pagination,
    },
    filters::{parse_listings_query, ListingsQuery},
    id_inventory,
};

const ACTIVE_UPDATE_WINDOW: Duration = Duration::minutes(5);
const RECENT_LISTING_WINDOW: Duration = Duration::hours(2);

pub fn routes(state: Arc<State>) -> BoxedFilter<(Response,)> {
    collection_route(Arc::clone(&state))
        .or(member_route(state))
        .unify()
        .boxed()
}

fn collection_route(state: Arc<State>) -> BoxedFilter<(Response,)> {
    warp::path!("api" / "v2" / "listings")
        .and(warp::path::end())
        .and(warp::get())
        .and(
            warp::query::<HashMap<String, String>>()
                .or(warp::any().map(HashMap::new))
                .unify(),
        )
        .and(warp::any().map(move || state.clone()))
        .and_then(collection)
        .boxed()
}

fn member_route(state: Arc<State>) -> BoxedFilter<(Response,)> {
    warp::path!("api" / "v2" / "listings" / u32)
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::any().map(move || state.clone()))
        .and_then(member)
        .boxed()
}

pub(crate) fn collection_route_for_tests() -> BoxedFilter<(Response,)> {
    warp::path!("api" / "v2" / "listings")
        .and(warp::path::end())
        .and(warp::get())
        .and(
            warp::query::<HashMap<String, String>>()
                .or(warp::any().map(HashMap::new))
                .unify(),
        )
        .and_then(collection_with_empty_documents)
        .boxed()
}

#[cfg(test)]
pub(crate) fn collection_response_from_raw_documents_for_tests(
    query: ListingsQuery,
    documents: Vec<Document>,
) -> Response {
    collection_response_from_raw_documents(query, documents)
}

async fn collection(
    query: HashMap<String, String>,
    state: Arc<State>,
) -> Result<Response, Infallible> {
    match parse_listings_query(&query) {
        Ok(query) => Ok(collection_response(query, state).await.into_response()),
        Err(error) => Ok(invalid_query_reply(error).into_response()),
    }
}

async fn collection_with_empty_documents(
    query: HashMap<String, String>,
) -> Result<Response, Infallible> {
    match parse_listings_query(&query) {
        Ok(query) => Ok(warp::reply::json(&collection_response_from_documents(query, []))
            .into_response()),
        Err(error) => Ok(invalid_query_reply(error).into_response()),
    }
}

pub(crate) fn member_route_for_tests(
    documents: Vec<QueriedListing>,
) -> BoxedFilter<(Response,)> {
    let documents = Arc::new(documents);

    warp::path!("api" / "v2" / "listings" / u32)
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::any().map(move || Arc::clone(&documents)))
        .and_then(member_from_documents)
        .boxed()
}

async fn member(id: u32, state: Arc<State>) -> Result<Response, Infallible> {
    Ok(member_response(id, state).await)
}

async fn member_from_documents(
    id: u32,
    documents: Arc<Vec<QueriedListing>>,
) -> Result<Response, Infallible> {
    Ok(member_response_from_documents(id, documents.iter()))
}

async fn member_response(id: u32, state: Arc<State>) -> Response {
    let aggregation = state.collection().aggregate(member_pipeline(id), None).await;

    match aggregation {
        Ok(mut cursor) => {
            let mut documents = Vec::new();

            while let Some(result) = cursor.next().await {
                match result {
                    Ok(document) => match mongodb::bson::from_document::<QueriedListing>(document) {
                        Ok(document) => documents.push(document),
                        Err(error) => {
                            eprintln!("{error:#?}");
                            return internal_error_reply().into_response();
                        }
                    },
                    Err(error) => {
                        eprintln!("{error:#?}");
                        return internal_error_reply().into_response();
                    }
                }
            }

            member_response_from_documents(id, documents.iter())
        }
        Err(error) => {
            eprintln!("{error:#?}");
            internal_error_reply().into_response()
        }
    }
}

fn member_pipeline(id: u32) -> Vec<Document> {
    vec![
        doc! {
            "$match": {
                "updated_at": { "$gte": Utc::now() - RECENT_LISTING_WINDOW },
                "listing.id": id,
                "listing.search_area": { "$bitsAllClear": SearchAreaFlags::PRIVATE.bits() as i32 },
            }
        },
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
                "minutes_since_update": {
                    "$divide": [
                        { "$subtract": ["$$NOW", "$updated_at"] },
                        60000,
                    ]
                },
                "updated_minute": {
                    "$dateTrunc": {
                        "date": "$updated_at",
                        "unit": "minute",
                        "binSize": 5,
                    }
                },
            }
        },
        doc! {
            "$sort": {
                "updated_at": -1,
            }
        },
    ]
}

pub(crate) fn member_response_from_documents<'a>(
    id: u32,
    documents: impl IntoIterator<Item = &'a QueriedListing>,
) -> Response {
    match resolve_listing_detail(id, documents) {
        Some(detail) => warp::reply::json(&ListingMemberResponse { data: detail }).into_response(),
        None => not_found_reply(id).into_response(),
    }
}

fn not_found_reply(id: u32) -> impl Reply {
    let mut details = Map::new();
    details.insert("id".into(), Value::from(id));

    warp::reply::with_status(
        warp::reply::json(&ErrorEnvelope::new(
            "not_found",
            "Listing not found",
            details,
        )),
        StatusCode::NOT_FOUND,
    )
}

fn invalid_query_reply(error: ErrorEnvelope) -> impl Reply {
    warp::reply::with_status(warp::reply::json(&error), StatusCode::BAD_REQUEST)
}

async fn collection_response(query: ListingsQuery, state: Arc<State>) -> Response {
    if query_demands_empty_collection(&query) {
        return warp::reply::json(&empty_collection_response(&query)).into_response();
    }

    let aggregation = state.collection().aggregate(collection_pipeline(&query), None).await;

    match aggregation {
        Ok(mut cursor) => {
            let mut documents = Vec::new();

            while let Some(result) = cursor.next().await {
                match result {
                    Ok(document) => match decode_queried_listing(document) {
                        Ok(document) => documents.push(document),
                        Err(()) => return internal_error_reply().into_response(),
                    },
                    Err(error) => {
                        eprintln!("{error:#?}");
                        return internal_error_reply().into_response();
                    }
                }
            }

            collection_response_from_decoded_documents(query, documents)
        }
        Err(error) => {
            eprintln!("{error:#?}");
            internal_error_reply().into_response()
        }
    }
}

fn internal_error_reply() -> impl Reply {
    warp::reply::with_status(
        warp::reply::json(&ErrorEnvelope::new(
            "internal_error",
            "Failed to load API v2 listings",
            Default::default(),
        )),
        StatusCode::INTERNAL_SERVER_ERROR,
    )
}

pub(crate) fn collection_pipeline(query: &ListingsQuery) -> Vec<Document> {
    let mut pipeline = vec![doc! {
        "$match": {
            "updated_at": { "$gte": Utc::now() - RECENT_LISTING_WINDOW },
            "listing.search_area": { "$bitsAllClear": SearchAreaFlags::PRIVATE.bits() as i32 },
        }
    }];

    if let Some(created_world_id) = query.created_world_id {
        pipeline.push(doc! {
            "$match": {
                "listing.created_world": created_world_id as i32,
            }
        });
    }

    if let Some(home_world_id) = query.home_world_id {
        pipeline.push(doc! {
            "$match": {
                "listing.home_world": home_world_id as i32,
            }
        });
    }

    if let Some(category_id) = query.category_id {
        pipeline.push(doc! {
            "$match": {
                "listing.category": category_id as i32,
            }
        });
    }

    if let Some(duty_id) = query.duty_id {
        pipeline.push(doc! {
            "$match": {
                "listing.duty": duty_id as i32,
            }
        });
    }

    if !query.job_ids.is_empty() {
        let job_conditions = query
            .job_ids
            .iter()
            .filter_map(|job_id| id_inventory::accepted_job_flag_bits(*job_id))
            .map(|job_flag_bits| {
                doc! {
                    "listing.slots": {
                        "$elemMatch": {
                            "accepting": {
                                "$bitsAllSet": job_flag_bits as i64
                            }
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        pipeline.push(doc! {
            "$match": {
                "$or": job_conditions,
            }
        });
    }

    pipeline.extend([
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
                "minutes_since_update": {
                    "$divide": [
                        { "$subtract": ["$$NOW", "$updated_at"] },
                        60000,
                    ]
                },
            }
        },
        doc! {
            "$match": {
                "$and": [
                    { "time_left": { "$gte": 0 } },
                    { "minutes_since_update": { "$lt": 5.0 } },
                ]
            }
        },
        doc! {
            "$sort": {
                "updated_at": -1,
            }
        },
        doc! {
            "$group": {
                "_id": "$listing.content_id_lower",
                "doc": { "$first": "$$ROOT" },
            }
        },
        doc! {
            "$replaceRoot": { "newRoot": "$doc" }
        },
        doc! {
            "$set": {
                "updated_minute": {
                    "$dateTrunc": {
                        "date": "$updated_at",
                        "unit": "minute",
                        "binSize": 5,
                    }
                }
            }
        },
        doc! {
            "$sort": {
                "updated_minute": -1,
                "listing.category": -1,
                "time_left": 1,
            }
        },
    ]);

    pipeline
}

pub(crate) fn collection_response_from_documents<'a>(
    query: ListingsQuery,
    documents: impl IntoIterator<Item = &'a QueriedListing>,
) -> ListingCollectionResponse {
    let filtered = documents
        .into_iter()
        .filter(|document| matches_query(document, &query))
        .filter_map(project_listing_summary)
        .collect::<Vec<_>>();

    paginated_collection_response(query, filtered)
}

fn paginated_collection_response(
    query: ListingsQuery,
    filtered: Vec<ListingSummary>,
) -> ListingCollectionResponse {
    let total = filtered.len();
    let total_pages = total_pages(total, query.per_page);
    let data = if query.page > total_pages && total > 0 {
        Vec::new()
    } else {
        filtered
            .into_iter()
            .skip((query.page - 1) * query.per_page)
            .take(query.per_page)
            .collect()
    };

    CollectionEnvelope {
        data,
        pagination: Pagination {
            total,
            page: query.page,
            per_page: query.per_page,
            total_pages,
        },
    }
}

fn empty_collection_response(query: &ListingsQuery) -> ListingCollectionResponse {
    CollectionEnvelope {
        data: Vec::new(),
        pagination: Pagination {
            total: 0,
            page: query.page,
            per_page: query.per_page,
            total_pages: 0,
        },
    }
}

fn total_pages(total: usize, per_page: usize) -> usize {
    if total == 0 {
        0
    } else {
        (total + per_page - 1) / per_page
    }
}

fn query_demands_empty_collection(query: &ListingsQuery) -> bool {
    query
        .created_world_id
        .is_some_and(|world_id| !id_inventory::world_ids().contains(&world_id))
        || query
            .home_world_id
            .is_some_and(|world_id| !id_inventory::world_ids().contains(&world_id))
        || query
            .category_id
            .is_some_and(|category_id| !id_inventory::CATEGORY_IDS.contains(&category_id))
        || query.duty_id.is_some_and(|duty_id| duty_id > u16::MAX as u32)
        || query
            .job_ids
            .iter()
            .any(|job_id| !id_inventory::job_ids().contains(job_id))
}

fn matches_query(document: &QueriedListing, query: &ListingsQuery) -> bool {
    let listing = match visible_listing(document) {
        Some(listing) => listing,
        None => return false,
    };

    query
        .created_world_id
        .is_none_or(|world_id| id_inventory::world_id(listing.created_world) == world_id)
        && query
            .home_world_id
            .is_none_or(|world_id| id_inventory::world_id(listing.home_world) == world_id)
        && query
            .category_id
            .is_none_or(|category_id| id_inventory::category_id(listing.category) == category_id)
        && query
            .duty_id
            .is_none_or(|duty_id| id_inventory::duty_id(listing.duty) == duty_id)
        && matches_job_ids(listing, &query.job_ids)
        && matches_search(listing, query.search.as_deref())
}

fn matches_job_ids(listing: &PartyFinderListing, job_ids: &[u32]) -> bool {
    job_ids.is_empty()
        || listing.slots.iter().any(|slot| {
            job_ids
                .iter()
                .any(|job_id| id_inventory::slot_accepts_job_id(slot.accepting, *job_id))
        })
}

fn collection_response_from_raw_documents(query: ListingsQuery, documents: Vec<Document>) -> Response {
    let mut decoded_documents = Vec::new();

    for document in documents {
        match decode_queried_listing(document) {
            Ok(document) => decoded_documents.push(document),
            Err(()) => return internal_error_reply().into_response(),
        }
    }

    collection_response_from_decoded_documents(query, decoded_documents)
}

fn collection_response_from_decoded_documents(
    query: ListingsQuery,
    documents: Vec<QueriedListing>,
) -> Response {
    warp::reply::json(&collection_response_from_documents(query, documents.iter())).into_response()
}

fn decode_queried_listing(document: Document) -> Result<QueriedListing, ()> {
    mongodb::bson::from_document::<QueriedListing>(document).map_err(|error| {
        eprintln!("{error:#?}");
    })
}

fn matches_search(listing: &PartyFinderListing, search: Option<&str>) -> bool {
    let Some(search) = search else {
        return true;
    };

    let search = search.to_lowercase();
    let player_name = listing
        .name
        .full_text(&crate::ffxiv::Language::ChineseSimplified)
        .to_lowercase();
    let description = listing
        .description
        .full_text(&crate::ffxiv::Language::ChineseSimplified)
        .to_lowercase();

    player_name.contains(&search) || description.contains(&search)
}

pub(crate) fn project_listing_summaries<'a>(
    documents: impl IntoIterator<Item = &'a QueriedListing>,
) -> Vec<ListingSummary> {
    documents
        .into_iter()
        .filter_map(project_listing_summary)
        .collect()
}

pub(crate) fn project_listing_summary(document: &QueriedListing) -> Option<ListingSummary> {
    let listing = visible_listing(document)?;

    Some(ListingSummary {
        id: listing.id,
        player_name: listing
            .name
            .full_text(&crate::ffxiv::Language::ChineseSimplified)
            .into(),
        description: listing
            .description
            .full_text(&crate::ffxiv::Language::ChineseSimplified)
            .into(),
        created_world_id: id_inventory::world_id(listing.created_world),
        home_world_id: id_inventory::world_id(listing.home_world),
        category_id: id_inventory::category_id(listing.category),
        duty_id: id_inventory::duty_id(listing.duty),
        duty_type_id: id_inventory::duty_type_id(listing.duty_type),
        min_item_level: listing.min_item_level,
        slots_filled: count_slots_filled(listing),
        slots_available: listing.slots_available,
        time_left_seconds: time_left_seconds(document.time_left),
        updated_at: document.updated_at.to_rfc3339(),
        is_cross_world: is_cross_world(listing),
        beginners_welcome: listing.beginners_welcome,
    })
}

pub(crate) fn resolve_listing_detail<'a>(
    id: u32,
    documents: impl IntoIterator<Item = &'a QueriedListing>,
) -> Option<ListingDetail> {
    let mut selected: Option<&QueriedListing> = None;

    for document in documents {
        let Some(listing) = visible_listing(document) else {
            continue;
        };

        if listing.id != id {
            continue;
        }

        match selected {
            Some(current) if document.updated_at <= current.updated_at => {}
            _ => selected = Some(document),
        }
    }

    selected.and_then(project_listing_detail)
}

pub(crate) fn project_listing_detail(document: &QueriedListing) -> Option<ListingDetail> {
    let listing = visible_listing(document)?;

    Some(ListingDetail {
        id: listing.id,
        player_name: listing
            .name
            .full_text(&crate::ffxiv::Language::ChineseSimplified)
            .into(),
        description: listing
            .description
            .full_text(&crate::ffxiv::Language::ChineseSimplified)
            .into(),
        created_world_id: id_inventory::world_id(listing.created_world),
        home_world_id: id_inventory::world_id(listing.home_world),
        category_id: id_inventory::category_id(listing.category),
        duty_id: id_inventory::duty_id(listing.duty),
        duty_type_id: id_inventory::duty_type_id(listing.duty_type),
        min_item_level: listing.min_item_level,
        slots_filled: count_slots_filled(listing),
        slots_available: listing.slots_available,
        time_left_seconds: time_left_seconds(document.time_left),
        updated_at: document.updated_at.to_rfc3339(),
        is_cross_world: is_cross_world(listing),
        beginners_welcome: listing.beginners_welcome,
        objective_ids: id_inventory::objective_ids(listing.objective),
        condition_ids: id_inventory::condition_ids(listing.conditions),
        loot_rule_id: id_inventory::loot_rule_id(listing.loot_rules),
        slots: project_slots(listing),
    })
}

fn visible_listing(document: &QueriedListing) -> Option<&PartyFinderListing> {
    let listing = &document.listing;
    (!listing.search_area.contains(SearchAreaFlags::PRIVATE) && is_active_listing(document))
        .then_some(listing)
}

fn is_active_listing(document: &QueriedListing) -> bool {
    document.time_left >= 0.0 && document.updated_at >= Utc::now() - ACTIVE_UPDATE_WINDOW
}

fn time_left_seconds(time_left: f64) -> u32 {
    time_left.max(0.0).floor() as u32
}

fn count_slots_filled(listing: &PartyFinderListing) -> usize {
    listing.jobs_present.iter().filter(|job| **job > 0).count()
}

fn is_cross_world(listing: &PartyFinderListing) -> bool {
    listing.search_area.contains(SearchAreaFlags::DATA_CENTRE)
}

fn project_slots(listing: &PartyFinderListing) -> Vec<ListingSlot> {
    listing
        .slots
        .iter()
        .take(listing.slots_available as usize)
        .enumerate()
        .map(|(index, slot)| {
            let filled_job_id = listing
                .jobs_present
                .get(index)
                .copied()
                .and_then(id_inventory::filled_job_id);
            let accepted_job_ids = filled_job_id
                .is_none()
                .then(|| id_inventory::accepted_job_ids(slot.accepting))
                .unwrap_or_default();
            let role_id = filled_job_id
                .and_then(id_inventory::role_id_for_job_id)
                .or_else(|| id_inventory::role_id_for_job_ids(&accepted_job_ids))
                .unwrap_or_default();

            ListingSlot {
                filled: filled_job_id.is_some(),
                role_id,
                filled_job_id,
                accepted_job_ids,
            }
        })
        .collect()
}

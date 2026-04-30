use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::contracts::ErrorEnvelope;

pub const DEFAULT_PAGE: usize = 1;
pub const DEFAULT_PER_PAGE: usize = 20;
pub const MAX_PER_PAGE: usize = 100;

const SUPPORTED_QUERY_FIELDS: &[&str] = &[
    "page",
    "per_page",
    "created_world_id",
    "home_world_id",
    "datacenter",
    "region",
    "category_id",
    "duty_id",
    "job_ids",
    "search",
];

const LEGACY_LABEL_FIELDS: &[(&str, &str)] = &[
    (
        "world",
        "world is not supported in v2; use created_world_id or home_world_id",
    ),
    (
        "category",
        "category is not supported in v2; use category_id",
    ),
    ("duty", "duty is not supported in v2; use duty_id"),
    ("jobs", "jobs is not supported in v2; use job_ids"),
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListingsQuery {
    pub page: usize,
    pub per_page: usize,
    pub created_world_id: Vec<u32>,
    pub home_world_id: Vec<u32>,
    pub datacenter: Option<String>,
    pub region: Option<String>,
    pub category_id: Option<u32>,
    pub duty_id: Option<u32>,
    pub job_ids: Vec<u32>,
    pub search: Option<String>,
}

impl Default for ListingsQuery {
    fn default() -> Self {
        Self {
            page: DEFAULT_PAGE,
            per_page: DEFAULT_PER_PAGE,
            created_world_id: Vec::new(),
            home_world_id: Vec::new(),
            datacenter: None,
            region: None,
            category_id: None,
            duty_id: None,
            job_ids: Vec::new(),
            search: None,
        }
    }
}

pub fn parse_listings_query(
    params: &HashMap<String, String>,
) -> Result<ListingsQuery, ErrorEnvelope> {
    for field in sorted_keys(params) {
        if let Some((_, message)) = LEGACY_LABEL_FIELDS
            .iter()
            .find(|(legacy, _)| field == *legacy)
        {
            return Err(ErrorEnvelope::invalid_query(field.clone(), *message));
        }

        if !SUPPORTED_QUERY_FIELDS
            .iter()
            .any(|supported| field == *supported)
        {
            return Err(ErrorEnvelope::invalid_query(
                field.clone(),
                format!("{field} is not a supported query parameter"),
            ));
        }
    }

    let mut query = ListingsQuery::default();

    if let Some(value) = params.get("page") {
        query.page = parse_page(value)?;
    }

    if let Some(value) = params.get("per_page") {
        query.per_page = parse_per_page(value)?;
    }

    query.created_world_id = parse_csv_u32s(params, "created_world_id")?;
    query.home_world_id = parse_csv_u32s(params, "home_world_id")?;

    query.datacenter = parse_csv_names(params, "datacenter")?;
    query.region = parse_csv_names(params, "region")?;

    query.category_id = parse_optional_u32(params, "category_id")?;
    query.duty_id = parse_optional_u32(params, "duty_id")?;

    if let Some(value) = params.get("job_ids") {
        query.job_ids = parse_job_ids(value)?;
    }

    if let Some(value) = params.get("search") {
        if !value.is_empty() {
            query.search = Some(value.clone());
        }
    }

    Ok(query)
}

fn sorted_keys(params: &HashMap<String, String>) -> Vec<String> {
    let mut keys = params.keys().cloned().collect::<Vec<_>>();
    keys.sort();
    keys
}

fn parse_page(value: &str) -> Result<usize, ErrorEnvelope> {
    let page = value
        .parse::<usize>()
        .map_err(|_| ErrorEnvelope::invalid_query("page", "page must be a positive integer"))?;

    if page == 0 {
        return Err(ErrorEnvelope::invalid_query(
            "page",
            "page must be a positive integer",
        ));
    }

    Ok(page)
}

fn parse_per_page(value: &str) -> Result<usize, ErrorEnvelope> {
    let per_page = value.parse::<usize>().map_err(|_| {
        ErrorEnvelope::invalid_query("per_page", "per_page must be between 1 and 100")
    })?;

    if !(1..=MAX_PER_PAGE).contains(&per_page) {
        return Err(ErrorEnvelope::invalid_query(
            "per_page",
            "per_page must be between 1 and 100",
        ));
    }

    Ok(per_page)
}

fn parse_optional_u32(
    params: &HashMap<String, String>,
    field: &'static str,
) -> Result<Option<u32>, ErrorEnvelope> {
    params
        .get(field)
        .map(|value| {
            value.parse::<u32>().map_err(|_| {
                ErrorEnvelope::invalid_query(field, format!("{field} must be an unsigned integer"))
            })
        })
        .transpose()
}

fn parse_job_ids(value: &str) -> Result<Vec<u32>, ErrorEnvelope> {
    parse_csv_u32s_impl(value, "job_ids")
}

fn parse_csv_u32s(
    params: &HashMap<String, String>,
    field: &'static str,
) -> Result<Vec<u32>, ErrorEnvelope> {
    let value = match params.get(field) {
        Some(v) => v,
        None => return Ok(Vec::new()),
    };

    if value.is_empty() {
        return Err(ErrorEnvelope::invalid_query(
            field,
            format!("{field} must be a comma-separated list of unsigned integers"),
        ));
    }

    parse_csv_u32s_impl(value, field)
}

fn parse_csv_u32s_impl(value: &str, field: &str) -> Result<Vec<u32>, ErrorEnvelope> {
    value
        .split(',')
        .map(|segment| {
            let trimmed = segment.trim();
            if trimmed.is_empty() {
                return Err(ErrorEnvelope::invalid_query(
                    field,
                    format!("{field} must be a comma-separated list of unsigned integers"),
                ));
            }

            trimmed.parse::<u32>().map_err(|_| {
                ErrorEnvelope::invalid_query(
                    field,
                    format!("{field} must be a comma-separated list of unsigned integers"),
                )
            })
        })
        .collect()
}

fn parse_csv_names(
    params: &HashMap<String, String>,
    field: &'static str,
) -> Result<Option<String>, ErrorEnvelope> {
    let value = match params.get(field) {
        Some(v) => v,
        None => return Ok(None),
    };

    if value.is_empty() {
        return Err(ErrorEnvelope::invalid_query(
            field,
            format!("{field} must be a comma-separated list of names"),
        ));
    }

    let names = value
        .split(',')
        .map(str::trim)
        .map(|segment| {
            if segment.is_empty() {
                Err(ErrorEnvelope::invalid_query(
                    field,
                    format!("{field} must be a comma-separated list of names"),
                ))
            } else {
                Ok(segment.to_owned())
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Some(names.join(",")))
}

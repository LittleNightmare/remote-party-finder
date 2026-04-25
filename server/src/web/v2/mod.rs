use std::sync::Arc;

use warp::{filters::BoxedFilter, Reply};

use crate::web::State;

pub mod contracts;
pub mod filters;
pub mod id_inventory;
pub mod listings;
pub mod lookups;

pub fn routes(state: Arc<State>) -> BoxedFilter<(impl Reply,)> {
    listings::routes(state)
}

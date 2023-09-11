use axum::routing::{Router, get};
use crate::api::work::get_all_work;

pub fn get_api_router() -> Router {
    Router::new()
        .route("/work", get(get_all_work))
}

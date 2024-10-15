use crate::handlers::example::{get_example, list_examples};
use axum::{routing::get, Router};

pub fn get_routes() -> Router {
    Router::new()
        .route("/examples", get(list_examples))
        .route("/examples/:id", get(get_example))
}

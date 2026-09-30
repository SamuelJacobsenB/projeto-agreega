use axum::{Router, routing::get};

use super::handler::Handler;
use crate::app::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(Handler::get_all_clients))
        .route("/:id", get(Handler::get_client_by_id))
        .route("/", axum::routing::post(Handler::create_client))
        .route("/:id", axum::routing::put(Handler::update_client))
        .route("/:id", axum::routing::delete(Handler::delete_client))
}

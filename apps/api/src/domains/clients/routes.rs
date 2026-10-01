use axum::{
    Router,
    routing::{delete, get, post, put},
};

use super::handler::Handler;
use crate::app::AppState;

pub fn authenticated_routes() -> Router<AppState> {
    Router::new().route("/me", get(Handler::get_my_client))
}

pub fn staff_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(Handler::get_all_clients))
        .route("/", post(Handler::create_client))
        .route("/{id}", get(Handler::get_client_by_id))
        .route("/{id}", put(Handler::update_client))
        .route("/{id}", delete(Handler::delete_client))
}

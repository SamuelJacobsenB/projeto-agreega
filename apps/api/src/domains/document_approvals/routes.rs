use axum::{
    Router,
    routing::{get, post, put},
};

use crate::app::AppState;

use super::handler::Handler;

pub fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/document/{document_id}", get(Handler::list_by_document_id))
        .route("/{id}", get(Handler::get_by_id))
        .route("/{id}/review", put(Handler::review))
}

pub fn staff_routes() -> Router<AppState> {
    Router::new().route("/", post(Handler::create))
}

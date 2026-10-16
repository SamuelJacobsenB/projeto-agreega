use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};

use super::handler::Handler;
use crate::app::AppState;

const MAX_MULTIPART_BODY_SIZE: usize = 20 * 1024 * 1024 + 64 * 1024;

pub fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/document/{document_id}", get(Handler::list_by_document_id))
        .route("/{id}", get(Handler::get_by_id))
        .route("/{id}/file", get(Handler::get_file))
}

pub fn staff_routes() -> Router<AppState> {
    Router::new().route(
        "/",
        post(Handler::create).layer(DefaultBodyLimit::max(MAX_MULTIPART_BODY_SIZE)),
    )
}

use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};

use super::handler::Handler;
use crate::app::AppState;
use crate::middlewares::rate_limit::{self, RateLimitPolicies};

pub fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/document/{document_id}", get(Handler::list_by_document_id))
        .route("/{id}", get(Handler::get_by_id))
        .route("/{id}/file", get(Handler::get_file))
}

pub fn staff_routes(
    policies: &RateLimitPolicies,
    max_upload_body_size_bytes: usize,
) -> Router<AppState> {
    Router::new().route(
        "/",
        rate_limit::protect_route(post(Handler::create), policies.upload.clone())
            .layer(DefaultBodyLimit::max(max_upload_body_size_bytes)),
    )
}

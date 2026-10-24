use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, patch, post},
};

use super::handler::Handler;
use crate::app::AppState;
use crate::middlewares::rate_limit::{self, RateLimitPolicies};

pub fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/stage/{stage_id}", get(Handler::list_stage_photos))
        .route("/task/{task_id}", get(Handler::list_task_photos))
        .route("/{id}", get(Handler::get_photo_by_id))
        .route("/{id}/file", get(Handler::get_file))
}

pub fn staff_routes(
    policies: &RateLimitPolicies,
    max_upload_body_size_bytes: usize,
) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            rate_limit::protect_route(post(Handler::create_photo), policies.upload.clone())
                .layer(DefaultBodyLimit::max(max_upload_body_size_bytes)),
        )
        .route(
            "/{id}",
            patch(Handler::update_description).delete(Handler::delete_photo),
        )
}

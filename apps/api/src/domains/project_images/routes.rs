use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{delete, get, patch, post},
};

use super::handler::Handler;
use crate::app::AppState;
use crate::middlewares::rate_limit::{self, RateLimitPolicies};

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/project/{project_id}", get(Handler::list_project_images))
        .route(
            "/project/{project_id}/cover",
            get(Handler::get_cover_by_project_id),
        )
        .route("/{id}", get(Handler::get_project_image_by_id))
        .route("/{id}/file", get(Handler::get_file))
}

pub fn staff_routes(
    policies: &RateLimitPolicies,
    max_upload_body_size_bytes: usize,
) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            rate_limit::protect_route(post(Handler::create_project_image), policies.upload.clone())
                .layer(DefaultBodyLimit::max(max_upload_body_size_bytes)),
        )
        .route(
            "/project/{project_id}/reorder",
            patch(Handler::reorder_project_images),
        )
        .route("/{id}", delete(Handler::delete_project_image))
}

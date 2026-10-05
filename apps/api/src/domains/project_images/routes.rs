use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use super::handler::Handler;
use crate::app::AppState;

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

pub fn staff_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(Handler::create_project_image))
        .route(
            "/project/{project_id}/reorder",
            patch(Handler::reorder_project_images),
        )
        .route("/{id}", delete(Handler::delete_project_image))
}

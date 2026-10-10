use axum::{
    Router,
    routing::{get, patch, post},
};

use super::handler::Handler;
use crate::app::AppState;

pub fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/project/{project_id}", get(Handler::list_project_photos))
        .route("/stage/{stage_id}", get(Handler::list_stage_photos))
        .route("/task/{task_id}", get(Handler::list_task_photos))
        .route("/{id}", get(Handler::get_photo_by_id))
        .route("/{id}/file", get(Handler::get_file))
}

pub fn staff_routes() -> Router<AppState> {
    Router::new().route("/", post(Handler::create_photo)).route(
        "/{id}",
        patch(Handler::update_description).delete(Handler::delete_photo),
    )
}

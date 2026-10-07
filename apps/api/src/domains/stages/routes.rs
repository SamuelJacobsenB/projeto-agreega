use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use super::handler::Handler;
use crate::app::AppState;

pub fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/project/{project_id}", get(Handler::list_project_stages))
        .route("/{id}", get(Handler::get_stage_by_id))
}

pub fn staff_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(Handler::create_stage))
        .route(
            "/project/{project_id}/reorder",
            patch(Handler::reorder_stages),
        )
        .route("/{id}", delete(Handler::delete_stage))
}

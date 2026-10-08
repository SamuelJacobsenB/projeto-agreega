use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use super::handler::Handler;
use crate::app::AppState;

pub fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/stage/{stage_id}", get(Handler::list_stage_tasks))
        .route("/{id}", get(Handler::get_task_by_id))
}

pub fn staff_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(Handler::create_task))
        .route("/stage/{stage_id}/reorder", patch(Handler::reorder_tasks))
        .route("/{id}", delete(Handler::delete_task))
}

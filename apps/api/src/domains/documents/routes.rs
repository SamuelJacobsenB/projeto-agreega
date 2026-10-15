use axum::{
    Router,
    routing::{delete, get, post, put},
};

use super::handler::Handler;
use crate::app::AppState;

pub fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/stage/{stage_id}", get(Handler::list_stage_documents))
        .route("/task/{task_id}", get(Handler::list_task_documents))
        .route("/{id}", get(Handler::get_document_by_id))
}

pub fn staff_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(Handler::create_document))
        .route("/{id}", put(Handler::update_document))
        .route("/{id}", delete(Handler::delete_document))
}

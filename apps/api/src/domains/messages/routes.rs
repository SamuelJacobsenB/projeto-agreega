use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::app::AppState;

use super::handler::Handler;

pub fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/project/{project_id}", get(Handler::list_by_project_id))
        .route("/", post(Handler::create))
        .route("/{id}", patch(Handler::update).delete(Handler::delete))
}

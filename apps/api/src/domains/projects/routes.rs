use axum::{
    Router,
    routing::{delete, get, post, put},
};

use super::handler::Handler;
use crate::app::AppState;

// REVISAR ROTAS E SUAS RESPECTIVAS AUTENTICAÇÕES

pub fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(Handler::list_projects))
        .route("/{id}", get(Handler::get_project_by_id))
}

pub fn staff_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(Handler::create_project))
        .route("/{id}", put(Handler::update_project))
        .route("/{id}", delete(Handler::delete_project))
}

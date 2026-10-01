use axum::{
    Router,
    routing::{delete, get, put},
};

use super::handler::Handler;
use crate::app::AppState;

pub fn authenticated_routes() -> Router<AppState> {
    Router::new().route("/me", get(Handler::get_my_user))
}

pub fn staff_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(Handler::get_all_users))
        .route("/{id}", get(Handler::get_user_by_id))
        .route("/{id}", put(Handler::update_user))
        .route("/{id}", delete(Handler::delete_user))
}

use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{app::AppState, domains::invitations::handler::Handler};

pub fn public_routes() -> Router<AppState> {
    Router::new().route("/accept", post(Handler::accept_invitation))
}

pub fn staff_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(Handler::list_invitations))
        .route("/", post(Handler::create_invitation))
        .route("/{id}", delete(Handler::delete_invitation))
}

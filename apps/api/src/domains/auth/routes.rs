use axum::{Router, routing::post};

use super::handler::Handler;
use crate::app::AppState;

pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(Handler::login))
        .route("/refresh", post(Handler::refresh))
        .route(
            "/password-reset/request",
            post(Handler::request_password_reset),
        )
        .route("/password-reset", post(Handler::reset_password))
}

pub fn authenticated_routes() -> Router<AppState> {
    Router::new().route("/logout", post(Handler::logout))
}

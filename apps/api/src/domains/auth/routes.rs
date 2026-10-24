use axum::{Router, routing::post};

use super::handler::Handler;
use crate::app::AppState;
use crate::middlewares::rate_limit::{self, RateLimitPolicies};

pub fn public_routes(policies: &RateLimitPolicies) -> Router<AppState> {
    Router::new()
        .route(
            "/login",
            rate_limit::protect_route(post(Handler::login), policies.login.clone()),
        )
        .route(
            "/refresh",
            rate_limit::protect_route(post(Handler::refresh), policies.session_refresh.clone()),
        )
        .route(
            "/password-reset/request",
            rate_limit::protect_route(
                post(Handler::request_password_reset),
                policies.password_reset_request.clone(),
            ),
        )
        .route(
            "/password-reset",
            rate_limit::protect_route(
                post(Handler::reset_password),
                policies.password_reset_complete.clone(),
            ),
        )
}

pub fn authenticated_routes() -> Router<AppState> {
    Router::new().route("/logout", post(Handler::logout))
}

use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{
    app::AppState,
    domains::invitations::handler::Handler,
    middlewares::rate_limit::{self, RateLimitPolicies},
};

pub fn public_routes(policies: &RateLimitPolicies) -> Router<AppState> {
    Router::new().route(
        "/accept",
        rate_limit::protect_route(
            post(Handler::accept_invitation),
            policies.invitation_accept.clone(),
        ),
    )
}

pub fn staff_routes(policies: &RateLimitPolicies) -> Router<AppState> {
    Router::new()
        .route("/", get(Handler::list_invitations))
        .route(
            "/",
            rate_limit::protect_route(
                post(Handler::create_invitation),
                policies.invitation_create.clone(),
            ),
        )
        .route("/{id}", delete(Handler::delete_invitation))
}

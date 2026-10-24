use axum::{
    Router,
    routing::{get, post},
};

use crate::app::AppState;
use crate::middlewares::rate_limit::{self, RateLimitPolicies};

use super::handler::Handler;

pub fn authenticated_routes(policies: &RateLimitPolicies) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            rate_limit::protect_route(post(Handler::create), policies.upload.clone()),
        )
        .route("/message/{message_id}", get(Handler::list_by_message_id))
        .route(
            "/message/{message_id}/file/{file_id}",
            get(Handler::get_file).delete(Handler::delete),
        )
}

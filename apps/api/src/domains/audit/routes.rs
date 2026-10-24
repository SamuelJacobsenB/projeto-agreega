use axum::{Router, routing::get};

use crate::app::AppState;

use super::handler::Handler;

pub fn admin_routes() -> Router<AppState> {
    Router::new().route("/", get(Handler::list))
}

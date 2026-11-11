use axum::{Router, routing::get};

use super::handler::Handler;
use crate::app::AppState;

pub fn authenticated_routes() -> Router<AppState> {
    Router::new().route("/", get(Handler::get_dashboard))
}

use axum::{
    Router,
    routing::{get, patch},
};

use crate::app::AppState;

use super::handler::Handler;

pub fn authenticated_routes() -> Router<AppState> {
    Router::new().route("/", get(Handler::list_for_user)).route(
        "/{id}",
        patch(Handler::update_read_state).delete(Handler::delete),
    )
}

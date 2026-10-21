use axum::{Router, routing::get};

use crate::app::AppState;

use super::handler::Handler;

pub fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::post(Handler::create))
        .route("/message/{message_id}", get(Handler::list_by_message_id))
        .route(
            "/message/{message_id}/file/{file_id}",
            get(Handler::get_file).delete(Handler::delete),
        )
}

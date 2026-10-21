use axum::{
    Router,
    routing::{get, put},
};

use crate::app::AppState;

use super::handler::Handler;

pub fn authenticated_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/document-version/{document_version_id}",
            get(Handler::list_by_document_version_id).post(Handler::create),
        )
        .route(
            "/document-version/{document_version_id}/comments/{id}",
            put(Handler::update).delete(Handler::delete),
        )
}

use axum::Router;

use crate::app::AppState;

pub fn create_router(state: AppState) -> Router {
    let api_v1 = Router::new();

    Router::new().nest("/api/v1", api_v1).with_state(state)
}

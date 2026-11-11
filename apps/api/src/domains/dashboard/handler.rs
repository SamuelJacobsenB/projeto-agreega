use axum::{extract::State, http::StatusCode};

use crate::{
    app::AppState,
    domains::dashboard::{dtos::DashboardResponseDto, service::Service},
    response::{ApiResponse, ApiResult},
    security::auth_user::AuthUser,
};

pub struct Handler;

impl Handler {
    pub async fn get_dashboard(
        State(state): State<AppState>,
        AuthUser { id, role }: AuthUser,
    ) -> ApiResult<DashboardResponseDto> {
        let dashboard = Service::get_dashboard(&state.pool, id, role).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Dashboard carregado com sucesso.", Some(dashboard)),
        ))
    }
}

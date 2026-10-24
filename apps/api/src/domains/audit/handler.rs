use axum::{
    extract::{Query, State},
    http::StatusCode,
};

use crate::{
    app::AppState,
    response::{ApiResponse, ApiResult},
};

use super::{
    dtos::{AuditLogResponseDto, ListAuditLogsQueryDto},
    service::Service,
};

pub struct Handler;

impl Handler {
    pub async fn list(
        State(state): State<AppState>,
        Query(query): Query<ListAuditLogsQueryDto>,
    ) -> ApiResult<Vec<AuditLogResponseDto>> {
        let logs = Service::list(&state.pool, &query).await?;

        let log_dtos = logs.into_iter().map(AuditLogResponseDto::from).collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Registros de auditoria encontrados.", Some(log_dtos)),
        ))
    }
}

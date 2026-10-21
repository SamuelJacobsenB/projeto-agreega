use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    response::{ApiResponse, ApiResult, ValidatedJson},
    security::auth_user::AuthUser,
};

use super::{
    dtos::{
        CreateDocumentApprovalRequestDto, DocumentApprovalResponseDto,
        ReviewDocumentApprovalRequestDto,
    },
    service::Service,
};

pub struct Handler;

impl Handler {
    pub async fn list_by_document_id(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(document_id): Path<Uuid>,
    ) -> ApiResult<Vec<DocumentApprovalResponseDto>> {
        let approvals =
            Service::list_by_document_id(&state.pool, document_id, user_id, role).await?;

        let approval_dtos = approvals
            .into_iter()
            .map(DocumentApprovalResponseDto::from)
            .collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Aprovações do documento encontradas.", Some(approval_dtos)),
        ))
    }

    pub async fn get_by_id(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(id): Path<Uuid>,
    ) -> ApiResult<DocumentApprovalResponseDto> {
        let approval = Service::get_by_id(&state.pool, id, user_id, role).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Aprovação encontrada.",
                Some(DocumentApprovalResponseDto::from(approval)),
            ),
        ))
    }

    pub async fn create(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        ValidatedJson(dto): ValidatedJson<CreateDocumentApprovalRequestDto>,
    ) -> ApiResult<DocumentApprovalResponseDto> {
        let approval = Service::create(&state.pool, &dto, user_id, role).await?;

        Ok((
            StatusCode::CREATED,
            ApiResponse::success(
                "Solicitação de aprovação criada.",
                Some(DocumentApprovalResponseDto::from(approval)),
            ),
        ))
    }

    pub async fn review(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(id): Path<Uuid>,
        ValidatedJson(dto): ValidatedJson<ReviewDocumentApprovalRequestDto>,
    ) -> ApiResult<DocumentApprovalResponseDto> {
        let approval = Service::review(&state.pool, id, &dto, user_id, role).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Revisão da aprovação registrada.",
                Some(DocumentApprovalResponseDto::from(approval)),
            ),
        ))
    }
}

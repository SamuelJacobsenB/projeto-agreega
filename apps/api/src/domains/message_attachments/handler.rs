use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    domains::message_attachments::{
        dtos::{CreateMessageAttachmentRequestDto, MessageAttachmentResponseDto},
        service::Service,
    },
    response::{ApiResponse, ApiResult, AppResult, ValidatedJson},
    security::auth_user::AuthUser,
};

pub struct Handler;

impl Handler {
    pub async fn list_by_message_id(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(message_id): Path<Uuid>,
    ) -> ApiResult<Vec<MessageAttachmentResponseDto>> {
        let attachments =
            Service::list_by_message_id(&state.pool, message_id, user_id, role).await?;

        let attachment_dtos = attachments
            .into_iter()
            .map(MessageAttachmentResponseDto::from)
            .collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Anexos da mensagem encontrados.", Some(attachment_dtos)),
        ))
    }

    pub async fn create(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        ValidatedJson(dto): ValidatedJson<CreateMessageAttachmentRequestDto>,
    ) -> ApiResult<MessageAttachmentResponseDto> {
        let attachment = Service::create(&state.pool, &dto, user_id, role).await?;

        Ok((
            StatusCode::CREATED,
            ApiResponse::success(
                "Arquivo anexado à mensagem.",
                Some(MessageAttachmentResponseDto::from(attachment)),
            ),
        ))
    }

    pub async fn delete(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path((message_id, file_id)): Path<(Uuid, Uuid)>,
    ) -> ApiResult<()> {
        Service::delete(&state.pool, message_id, file_id, user_id, role).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Anexo removido da mensagem.", None),
        ))
    }

    pub async fn get_file(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path((message_id, file_id)): Path<(Uuid, Uuid)>,
    ) -> AppResult<Response> {
        Service::get_file(&state.pool, message_id, file_id, user_id, role).await
    }
}

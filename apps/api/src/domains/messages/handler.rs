use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    domains::messages::{
        dtos::{CreateMessageRequestDto, MessageResponseDto, UpdateMessageContentRequestDto},
        service::Service,
    },
    response::{ApiResponse, ApiResult, ValidatedJson},
    security::auth_user::AuthUser,
};

pub struct Handler;

impl Handler {
    pub async fn list_by_project_id(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(project_id): Path<Uuid>,
    ) -> ApiResult<Vec<MessageResponseDto>> {
        let messages = Service::list_by_project_id(&state.pool, project_id, user_id, role).await?;

        let message_dtos = messages.into_iter().map(MessageResponseDto::from).collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Mensagens do projeto encontradas.", Some(message_dtos)),
        ))
    }

    pub async fn create(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        ValidatedJson(dto): ValidatedJson<CreateMessageRequestDto>,
    ) -> ApiResult<MessageResponseDto> {
        let message = Service::create(&state.pool, &dto, user_id, role).await?;

        Ok((
            StatusCode::CREATED,
            ApiResponse::success(
                "Mensagem enviada com sucesso.",
                Some(MessageResponseDto::from(message)),
            ),
        ))
    }

    pub async fn update(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(id): Path<Uuid>,
        ValidatedJson(dto): ValidatedJson<UpdateMessageContentRequestDto>,
    ) -> ApiResult<MessageResponseDto> {
        let message = Service::update(&state.pool, id, &dto, user_id, role).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Mensagem atualizada com sucesso.",
                Some(MessageResponseDto::from(message)),
            ),
        ))
    }

    pub async fn delete(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(id): Path<Uuid>,
    ) -> ApiResult<()> {
        Service::delete(&state.pool, id, user_id, role).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Mensagem excluída com sucesso.", None),
        ))
    }
}

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
        CreateDocumentCommentRequestDto, DocumentCommentResponseDto,
        UpdateDocumentCommentRequestDto,
    },
    service::Service,
};

pub struct Handler;

impl Handler {
    pub async fn list_by_document_version_id(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(document_version_id): Path<Uuid>,
    ) -> ApiResult<Vec<DocumentCommentResponseDto>> {
        let comments =
            Service::list_by_document_version_id(&state.pool, document_version_id, user_id, role)
                .await?;

        let comment_dtos = comments
            .into_iter()
            .map(DocumentCommentResponseDto::from)
            .collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Comentários da versão encontrados.", Some(comment_dtos)),
        ))
    }

    pub async fn create(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(document_version_id): Path<Uuid>,
        ValidatedJson(dto): ValidatedJson<CreateDocumentCommentRequestDto>,
    ) -> ApiResult<DocumentCommentResponseDto> {
        let comment =
            Service::create(&state.pool, document_version_id, &dto, user_id, role).await?;

        Ok((
            StatusCode::CREATED,
            ApiResponse::success(
                "Comentário criado com sucesso.",
                Some(DocumentCommentResponseDto::from(comment)),
            ),
        ))
    }

    pub async fn update(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path((document_version_id, id)): Path<(Uuid, Uuid)>,
        ValidatedJson(dto): ValidatedJson<UpdateDocumentCommentRequestDto>,
    ) -> ApiResult<DocumentCommentResponseDto> {
        let comment =
            Service::update(&state.pool, document_version_id, id, &dto, user_id, role).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Comentário atualizado com sucesso.",
                Some(DocumentCommentResponseDto::from(comment)),
            ),
        ))
    }

    pub async fn delete(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path((document_version_id, id)): Path<(Uuid, Uuid)>,
    ) -> ApiResult<()> {
        Service::delete(&state.pool, document_version_id, id, user_id, role).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Comentário excluído com sucesso.", None),
        ))
    }
}

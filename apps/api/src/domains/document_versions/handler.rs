use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    response::{ApiResponse, ApiResult, AppResult, ValidatedMultipart},
    security::auth_user::AuthUser,
};

use super::{
    dtos::{CreateDocumentVersionRequestDto, DocumentVersionResponseDto},
    service::Service,
};

pub struct Handler;

impl Handler {
    pub async fn list_by_document_id(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(document_id): Path<Uuid>,
    ) -> ApiResult<Vec<DocumentVersionResponseDto>> {
        let versions =
            Service::get_document_versions_by_document_id(&state.pool, document_id, user_id, role)
                .await?;

        let version_dtos = versions
            .into_iter()
            .map(DocumentVersionResponseDto::from)
            .collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Versões do documento encontradas.", Some(version_dtos)),
        ))
    }

    pub async fn get_by_id(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(id): Path<Uuid>,
    ) -> ApiResult<DocumentVersionResponseDto> {
        let version = Service::get_document_version_by_id(&state.pool, id, user_id, role).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Versão de documento encontrada.",
                Some(DocumentVersionResponseDto::from(version)),
            ),
        ))
    }

    pub async fn create(
        State(state): State<AppState>,
        AuthUser {
            id: uploaded_by,
            role,
        }: AuthUser,
        ValidatedMultipart { data, file }: ValidatedMultipart<CreateDocumentVersionRequestDto>,
    ) -> ApiResult<DocumentVersionResponseDto> {
        let version = Service::create_document_version(
            &state.pool,
            &data,
            file,
            uploaded_by,
            role,
            state.config.max_upload_size_bytes,
        )
        .await?;

        Ok((
            StatusCode::CREATED,
            ApiResponse::success(
                "Nova versão do documento criada.",
                Some(DocumentVersionResponseDto::from(version)),
            ),
        ))
    }

    pub async fn get_file(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(id): Path<Uuid>,
    ) -> AppResult<Response> {
        Service::get_file(&state.pool, id, user_id, role).await
    }
}

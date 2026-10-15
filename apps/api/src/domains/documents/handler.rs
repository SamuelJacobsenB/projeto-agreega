use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    domains::documents::{
        dtos::{CreateDocumentRequestDto, DocumentResponseDto, UpdateDocumentRequestDto},
        service::Service,
    },
    response::{ApiResponse, ApiResult, ValidatedJson},
    security::auth_user::AuthUser,
};

pub struct Handler;

impl Handler {
    pub async fn list_stage_documents(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(stage_id): Path<Uuid>,
    ) -> ApiResult<Vec<DocumentResponseDto>> {
        let documents = Service::list_stage_documents(&state.pool, stage_id, user_id, role).await?;

        let document_dtos = documents
            .into_iter()
            .map(DocumentResponseDto::from)
            .collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Documentos do estágio encontrados.", Some(document_dtos)),
        ))
    }

    pub async fn list_task_documents(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(task_id): Path<Uuid>,
    ) -> ApiResult<Vec<DocumentResponseDto>> {
        let documents = Service::list_task_documents(&state.pool, task_id, user_id, role).await?;

        let document_dtos = documents
            .into_iter()
            .map(DocumentResponseDto::from)
            .collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Documentos da tarefa encontrados.", Some(document_dtos)),
        ))
    }

    pub async fn get_document_by_id(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(id): Path<Uuid>,
    ) -> ApiResult<DocumentResponseDto> {
        let document = Service::get_document_by_id(&state.pool, id, user_id, role).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Documento encontrado.",
                Some(DocumentResponseDto::from(document)),
            ),
        ))
    }

    pub async fn create_document(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        ValidatedJson(dto): ValidatedJson<CreateDocumentRequestDto>,
    ) -> ApiResult<DocumentResponseDto> {
        let document = Service::create_document(&state.pool, &dto, user_id, role).await?;

        Ok((
            StatusCode::CREATED,
            ApiResponse::success(
                "Documento criado com sucesso.",
                Some(DocumentResponseDto::from(document)),
            ),
        ))
    }

    pub async fn update_document(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(id): Path<Uuid>,
        ValidatedJson(dto): ValidatedJson<UpdateDocumentRequestDto>,
    ) -> ApiResult<DocumentResponseDto> {
        let document = Service::update_document(&state.pool, id, &dto, user_id, role).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Documento atualizado com sucesso.",
                Some(DocumentResponseDto::from(document)),
            ),
        ))
    }

    pub async fn delete_document(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(id): Path<Uuid>,
    ) -> ApiResult<()> {
        Service::delete_document(&state.pool, id, user_id, role).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Documento excluído com sucesso.", None),
        ))
    }
}

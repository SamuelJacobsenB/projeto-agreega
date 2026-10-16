use axum::response::Response;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::{
        document_versions::{
            dtos::CreateDocumentVersionRequestDto, models::DocumentVersion, repository::Repository,
        },
        documents::service::Service as DocumentService,
        files::{service::Service as FileService, types::UploadedFile},
        users::models::UserRole,
    },
    infrastructure::storage::StorageService,
    response::{AppError, AppResult, file_response},
};

pub const DOCUMENT_MIME_TYPES: &[&str] = &[
    "application/pdf",
    "application/msword",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "application/vnd.ms-excel",
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "application/vnd.ms-powerpoint",
    "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    "application/vnd.oasis.opendocument.text",
    "application/vnd.oasis.opendocument.spreadsheet",
    "application/vnd.oasis.opendocument.presentation",
    "text/plain",
    "text/csv",
    "application/rtf",
    "text/rtf",
];

pub struct Service;

impl Service {
    pub async fn get_document_versions_by_document_id(
        pool: &PgPool,
        document_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Vec<DocumentVersion>> {
        DocumentService::get_document_by_id(pool, document_id, user_id, role).await?;
        Repository::find_by_document_id(pool, document_id).await
    }

    pub async fn get_document_version_by_id(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<DocumentVersion> {
        let document_version = Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Versão de documento não encontrada.".to_string()))?;
        DocumentService::get_document_by_id(pool, document_version.document_id, user_id, role)
            .await?;

        Ok(document_version)
    }

    pub async fn create_document_version(
        pool: &PgPool,
        dto: &CreateDocumentVersionRequestDto,
        file: UploadedFile,
        uploaded_by: Uuid,
        role: UserRole,
    ) -> AppResult<DocumentVersion> {
        DocumentService::get_document_by_id(pool, dto.document_id, uploaded_by, role).await?;

        let file = FileService::create_file(
            pool,
            "document-versions",
            &file.original_name,
            &file.content,
            DOCUMENT_MIME_TYPES,
            uploaded_by,
        )
        .await?;

        match Repository::create(pool, dto.document_id, file.id, uploaded_by).await {
            Ok(document_version) => Ok(document_version),
            Err(error) => {
                let _ = FileService::delete_file(pool, file.id).await;
                Err(error)
            }
        }
    }

    pub async fn get_file(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Response> {
        let document_version = Self::get_document_version_by_id(pool, id, user_id, role).await?;
        let file = FileService::get_file_by_id(pool, document_version.file_id).await?;
        let content = StorageService::read_file(&file.storage_key).await?;

        file_response::private_file(content, &file.mime_type)
    }
}

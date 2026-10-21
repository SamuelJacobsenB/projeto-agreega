use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::{
        document_approvals::{
            dtos::{CreateDocumentApprovalRequestDto, ReviewDocumentApprovalRequestDto},
            models::{ApprovalStatus, DocumentApproval},
            repository::Repository,
        },
        document_versions::service::Service as DocumentVersionService,
        documents::service::Service as DocumentService,
        users::models::UserRole,
    },
    response::{AppError, AppResult},
};

pub struct Service;

impl Service {
    pub async fn list_by_document_id(
        pool: &PgPool,
        document_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Vec<DocumentApproval>> {
        DocumentService::get_document_by_id(pool, document_id, user_id, role).await?;
        Repository::find_by_document_id(pool, document_id).await
    }

    pub async fn get_by_id(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<DocumentApproval> {
        let approval = Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Aprovação não encontrada.".to_string()))?;
        DocumentService::get_document_by_id(pool, approval.document_id, user_id, role).await?;
        Ok(approval)
    }

    pub async fn create(
        pool: &PgPool,
        dto: &CreateDocumentApprovalRequestDto,
        requested_by: Uuid,
        role: UserRole,
    ) -> AppResult<DocumentApproval> {
        let version = DocumentVersionService::get_document_version_by_id(
            pool,
            dto.document_version_id,
            requested_by,
            role,
        )
        .await?;

        if version.document_id != dto.document_id {
            return Err(AppError::BadRequest(
                "A versão informada não pertence ao documento informado.".to_string(),
            ));
        }

        let document =
            DocumentService::get_document_by_id(pool, dto.document_id, requested_by, role).await?;

        if version.version != document.current_version {
            return Err(AppError::BadRequest(
                "A aprovação deve ser solicitada para a versão atual do documento.".to_string(),
            ));
        }

        Repository::create(
            pool,
            dto.document_id,
            dto.document_version_id,
            requested_by,
            dto.comment.as_deref(),
        )
        .await
    }

    pub async fn review(
        pool: &PgPool,
        id: Uuid,
        dto: &ReviewDocumentApprovalRequestDto,
        reviewed_by: Uuid,
        role: UserRole,
    ) -> AppResult<DocumentApproval> {
        let approval = Self::get_by_id(pool, id, reviewed_by, role).await?;

        if approval.status != ApprovalStatus::Pending {
            return Err(AppError::Conflict(
                "Esta aprovação já foi revisada.".to_string(),
            ));
        }

        if approval.requested_by == reviewed_by {
            return Err(AppError::Forbidden(
                "Quem solicitou a aprovação não pode revisar a própria solicitação.".to_string(),
            ));
        }

        if dto.status == ApprovalStatus::Pending {
            return Err(AppError::BadRequest(
                "A revisão deve aprovar, solicitar alterações ou rejeitar.".to_string(),
            ));
        }

        Repository::review(pool, id, reviewed_by, dto.status, dto.comment.as_deref()).await
    }
}

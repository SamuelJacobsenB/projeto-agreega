use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::{
        document_comments::{
            dtos::{CreateDocumentCommentRequestDto, UpdateDocumentCommentRequestDto},
            models::DocumentComment,
            repository::Repository,
        },
        document_versions::service::Service as DocumentVersionService,
        users::models::UserRole,
    },
    response::{AppError, AppResult},
};

pub struct Service;

impl Service {
    pub async fn list_by_document_version_id(
        pool: &PgPool,
        document_version_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Vec<DocumentComment>> {
        DocumentVersionService::get_document_version_by_id(
            pool,
            document_version_id,
            user_id,
            role,
        )
        .await?;
        Repository::find_by_document_version_id(pool, document_version_id).await
    }

    pub async fn create(
        pool: &PgPool,
        document_version_id: Uuid,
        dto: &CreateDocumentCommentRequestDto,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<DocumentComment> {
        DocumentVersionService::get_document_version_by_id(
            pool,
            document_version_id,
            user_id,
            role,
        )
        .await?;

        Repository::create(
            pool,
            document_version_id,
            user_id,
            &dto.content,
            dto.page,
            dto.position_x,
            dto.position_y,
        )
        .await
    }

    pub async fn update(
        pool: &PgPool,
        document_version_id: Uuid,
        id: Uuid,
        dto: &UpdateDocumentCommentRequestDto,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<DocumentComment> {
        if dto.content.is_none()
            && dto.page.is_none()
            && dto.position_x.is_none()
            && dto.position_y.is_none()
        {
            return Err(AppError::BadRequest(
                "Informe ao menos um campo para atualizar.".to_string(),
            ));
        }

        let comment =
            Self::get_authorized_comment(pool, document_version_id, id, user_id, role).await?;

        Repository::update(pool, comment.id, dto).await
    }

    pub async fn delete(
        pool: &PgPool,
        document_version_id: Uuid,
        id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<()> {
        Self::get_authorized_comment(pool, document_version_id, id, user_id, role).await?;
        Repository::delete(pool, id).await
    }

    async fn get_authorized_comment(
        pool: &PgPool,
        document_version_id: Uuid,
        id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<DocumentComment> {
        let comment = Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Comentário não encontrado.".to_string()))?;

        if comment.document_version_id != document_version_id {
            return Err(AppError::NotFound("Comentário não encontrado.".to_string()));
        }

        DocumentVersionService::get_document_version_by_id(
            pool,
            document_version_id,
            user_id,
            role,
        )
        .await?;

        if comment.user_id != user_id && role == UserRole::Client {
            return Err(AppError::Forbidden(
                "Você não tem permissão para alterar este comentário.".to_string(),
            ));
        }

        Ok(comment)
    }
}

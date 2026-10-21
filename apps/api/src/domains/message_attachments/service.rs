use axum::response::Response;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::{
        files::service::Service as FileService,
        message_attachments::{
            dtos::CreateMessageAttachmentRequestDto, models::MessageAttachment,
            repository::Repository,
        },
        messages::repository::Repository as MessageRepository,
        projects::authorization::authorize_project_access,
        users::models::UserRole,
    },
    infrastructure::storage::StorageService,
    response::{AppError, AppResult, file_response},
};

pub struct Service;

impl Service {
    pub async fn list_by_message_id(
        pool: &PgPool,
        message_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Vec<MessageAttachment>> {
        Self::get_accessible_message(pool, message_id, user_id, role).await?;
        Repository::find_by_message_id(pool, message_id).await
    }

    pub async fn create(
        pool: &PgPool,
        dto: &CreateMessageAttachmentRequestDto,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<MessageAttachment> {
        let message = Self::get_accessible_message(pool, dto.message_id, user_id, role).await?;
        Self::ensure_can_modify_message(&message.sender_id, user_id, role)?;

        let file = FileService::get_file_by_id(pool, dto.file_id).await?;
        if role == UserRole::Client && file.uploaded_by != user_id {
            return Err(AppError::Forbidden(
                "Você não tem permissão para anexar este arquivo.".to_string(),
            ));
        }

        Repository::create(pool, dto.message_id, dto.file_id).await
    }

    pub async fn delete(
        pool: &PgPool,
        message_id: Uuid,
        file_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<()> {
        let message = Self::get_accessible_message(pool, message_id, user_id, role).await?;
        Self::ensure_can_modify_message(&message.sender_id, user_id, role)?;
        Repository::delete(pool, message_id, file_id).await
    }

    pub async fn get_file(
        pool: &PgPool,
        message_id: Uuid,
        file_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Response> {
        Self::get_accessible_message(pool, message_id, user_id, role).await?;
        if !Repository::exists(pool, message_id, file_id).await? {
            return Err(AppError::NotFound("Anexo não encontrado.".to_string()));
        }

        let file = FileService::get_file_by_id(pool, file_id).await?;
        let content = StorageService::read_file(&file.storage_key).await?;
        file_response::private_file(content, &file.mime_type)
    }

    async fn get_accessible_message(
        pool: &PgPool,
        message_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<crate::domains::messages::models::Message> {
        let message = MessageRepository::find_by_id(pool, message_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Mensagem não encontrada.".to_string()))?;
        authorize_project_access(pool, message.project_id, user_id, role).await?;
        Ok(message)
    }

    fn ensure_can_modify_message(sender_id: &Uuid, user_id: Uuid, role: UserRole) -> AppResult<()> {
        if role == UserRole::Client && *sender_id != user_id {
            return Err(AppError::Forbidden(
                "Você não tem permissão para alterar anexos desta mensagem.".to_string(),
            ));
        }
        Ok(())
    }
}

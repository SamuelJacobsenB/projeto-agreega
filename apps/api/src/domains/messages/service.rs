use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::{
        messages::{
            dtos::{CreateMessageRequestDto, UpdateMessageContentRequestDto},
            models::Message,
            repository::Repository,
        },
        projects::authorization::authorize_project_access,
        users::models::UserRole,
    },
    response::{AppError, AppResult},
};

pub struct Service;

impl Service {
    pub async fn list_by_project_id(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Vec<Message>> {
        authorize_project_access(pool, project_id, user_id, role).await?;
        Repository::find_by_project_id(pool, project_id).await
    }

    pub async fn create(
        pool: &PgPool,
        dto: &CreateMessageRequestDto,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Message> {
        authorize_project_access(pool, dto.project_id, user_id, role).await?;
        Repository::create(pool, dto, user_id).await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        dto: &UpdateMessageContentRequestDto,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Message> {
        let message = Self::get_authorized_message(pool, id, user_id, role).await?;
        Repository::update(pool, message.id, &dto.content).await
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid, role: UserRole) -> AppResult<()> {
        Self::get_authorized_message(pool, id, user_id, role).await?;
        Repository::delete(pool, id).await
    }

    async fn get_authorized_message(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Message> {
        let message = Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Mensagem não encontrada.".to_string()))?;

        authorize_project_access(pool, message.project_id, user_id, role).await?;

        if message.sender_id != user_id && role == UserRole::Client {
            return Err(AppError::Forbidden(
                "Você não tem permissão para alterar esta mensagem.".to_string(),
            ));
        }

        Ok(message)
    }
}

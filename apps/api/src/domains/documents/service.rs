use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::{
        documents::{
            dtos::{CreateDocumentRequestDto, UpdateDocumentRequestDto},
            models::Document,
            repository::Repository,
        },
        projects::authorization::authorize_project_access,
        stages::service::Service as StageService,
        tasks::service::Service as TaskService,
        users::models::UserRole,
    },
    response::{AppError, AppResult},
};

pub struct Service;

impl Service {
    pub async fn list_stage_documents(
        pool: &PgPool,
        stage_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Vec<Document>> {
        StageService::get_stage_by_id(pool, stage_id, user_id, role).await?;
        Repository::find_by_stage_id(pool, stage_id).await
    }

    pub async fn list_task_documents(
        pool: &PgPool,
        task_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Vec<Document>> {
        TaskService::get_task_by_id(pool, task_id, user_id, role).await?;
        Repository::find_by_task_id(pool, task_id).await
    }

    pub async fn get_document_by_id(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Document> {
        let document = Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Documento não encontrado.".to_string()))?;
        authorize_project_access(pool, document.project_id, user_id, role).await?;
        Ok(document)
    }

    pub async fn create_document(
        pool: &PgPool,
        dto: &CreateDocumentRequestDto,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Document> {
        let stage = StageService::get_stage_by_id(pool, dto.stage_id, user_id, role).await?;
        if stage.project_id != dto.project_id {
            return Err(AppError::BadRequest(
                "O estágio informado não pertence ao projeto informado.".to_string(),
            ));
        }

        if let Some(task_id) = dto.task_id {
            let task = TaskService::get_task_by_id(pool, task_id, user_id, role).await?;
            if task.stage_id != dto.stage_id {
                return Err(AppError::BadRequest(
                    "A tarefa informada não pertence ao estágio informado.".to_string(),
                ));
            }
        }

        Repository::create(pool, dto, user_id).await
    }

    pub async fn update_document(
        pool: &PgPool,
        id: Uuid,
        dto: &UpdateDocumentRequestDto,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Document> {
        Self::get_document_by_id(pool, id, user_id, role).await?;

        if dto.name.is_none() && dto.document_type.is_none() && dto.status.is_none() {
            return Err(AppError::BadRequest(
                "Informe ao menos um campo para atualizar.".to_string(),
            ));
        }

        Repository::update(pool, id, dto).await
    }

    pub async fn delete_document(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<()> {
        Self::get_document_by_id(pool, id, user_id, role).await?;
        Repository::delete(pool, id).await
    }
}

use std::collections::HashSet;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::{
        stages::service::Service as StageService,
        tasks::{
            dtos::{CreateTaskRequestDto, ReorderTasksRequestDto},
            models::Task,
            repository::Repository,
        },
    },
    response::{AppError, AppResult},
};

pub struct Service;

impl Service {
    pub async fn list_stage_tasks(pool: &PgPool, stage_id: Uuid) -> AppResult<Vec<Task>> {
        StageService::get_stage_by_id(pool, stage_id).await?;
        Repository::find_by_stage_id(pool, stage_id).await
    }

    pub async fn get_task_by_id(pool: &PgPool, id: Uuid) -> AppResult<Task> {
        Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Tarefa não encontrada.".to_string()))
    }

    pub async fn create_task(pool: &PgPool, dto: &CreateTaskRequestDto) -> AppResult<Task> {
        StageService::get_stage_by_id(pool, dto.stage_id).await?;
        Repository::create(pool, dto).await
    }

    pub async fn reorder_tasks(
        pool: &PgPool,
        stage_id: Uuid,
        dto: &ReorderTasksRequestDto,
    ) -> AppResult<()> {
        let current_tasks = Self::list_stage_tasks(pool, stage_id).await?;

        if current_tasks.len() != dto.tasks.len() {
            return Err(AppError::BadRequest(
                "A quantidade de tarefas enviada não corresponde aos tarefas do estágio."
                    .to_string(),
            ));
        }

        let current_ids: HashSet<Uuid> = current_tasks.iter().map(|stage| stage.id).collect();

        let requested_ids: HashSet<Uuid> = dto.tasks.iter().copied().collect();

        if current_ids != requested_ids {
            return Err(AppError::BadRequest(
                "As tarefas enviados não correspondem às tarefas do estágio.".to_string(),
            ));
        }

        Repository::reorder(pool, stage_id, dto).await
    }

    pub async fn delete_task(pool: &PgPool, id: Uuid) -> AppResult<()> {
        let task = Self::get_task_by_id(pool, id).await?;
        Repository::delete(pool, id, task.stage_id, task.order).await
    }
}

use std::collections::HashSet;

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::{
        projects::{authorization::authorize_project_access, service::Service as ProjectService},
        stages::{
            dtos::{CreateStageRequestDto, ReorderStagesRequestDto},
            models::Stage,
            repository::Repository,
        },
        users::models::UserRole,
    },
    response::{AppError, AppResult},
};

pub struct Service;

impl Service {
    pub async fn list_project_stages(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Vec<Stage>> {
        ProjectService::get_project_by_id(pool, project_id).await?;
        authorize_project_access(pool, project_id, user_id, role).await?;
        Repository::find_by_project_id(pool, project_id).await
    }

    pub async fn get_stage_by_id(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Stage> {
        let stage = Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Estágio não encontrado.".to_string()))?;
        authorize_project_access(pool, stage.project_id, user_id, role).await?;
        Ok(stage)
    }

    pub async fn create_stage(pool: &PgPool, dto: &CreateStageRequestDto) -> AppResult<Stage> {
        ProjectService::get_project_by_id(pool, dto.project_id).await?;
        Repository::create(pool, dto).await
    }

    pub async fn reorder_stages(
        pool: &PgPool,
        project_id: Uuid,
        dto: &ReorderStagesRequestDto,
    ) -> AppResult<()> {
        let current_stages = Repository::find_by_project_id(pool, project_id).await?;

        if current_stages.len() != dto.stages.len() {
            return Err(AppError::BadRequest(
                "A quantidade de estágios enviada não corresponde aos estágios do projeto."
                    .to_string(),
            ));
        }

        let current_ids: HashSet<Uuid> = current_stages.iter().map(|stage| stage.id).collect();

        let requested_ids: HashSet<Uuid> = dto.stages.iter().copied().collect();

        if current_ids != requested_ids {
            return Err(AppError::BadRequest(
                "Os estágios enviados não correspondem aos estágios do projeto.".to_string(),
            ));
        }

        Repository::reorder(pool, project_id, dto).await
    }

    pub async fn delete_stage(pool: &PgPool, id: Uuid) -> AppResult<()> {
        let stage = Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Estágio não encontrado.".to_string()))?;
        Repository::delete(pool, id, stage.project_id, stage.order).await
    }
}

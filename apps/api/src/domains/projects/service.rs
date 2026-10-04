use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::clients::service::Service as ClientService,
    domains::projects::{
        dtos::{CreateProjectRequestDto, UpdateProjectRequestDto},
        models::Project,
        repository::Repository,
    },
    response::{AppError, AppResult},
};

pub struct Service;

impl Service {
    pub async fn list_projects(pool: &PgPool) -> AppResult<Vec<Project>> {
        Repository::find_all(pool).await
    }

    pub async fn get_project_by_id(pool: &PgPool, id: Uuid) -> AppResult<Project> {
        Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Projeto não encontrado.".to_string()))
    }

    pub async fn create_project(
        pool: &PgPool,
        dto: &CreateProjectRequestDto,
    ) -> AppResult<Project> {
        ClientService::get_client_by_id(pool, dto.client_id).await?;

        Repository::create(pool, dto).await
    }

    pub async fn update_project(
        pool: &PgPool,
        id: Uuid,
        dto: &UpdateProjectRequestDto,
    ) -> AppResult<Project> {
        Repository::update(pool, id, dto).await
    }

    pub async fn delete_project(pool: &PgPool, id: Uuid) -> AppResult<()> {
        Repository::delete(pool, id).await
    }
}

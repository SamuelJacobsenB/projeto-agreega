use axum::response::Response;
use sqlx::PgPool;
use uuid::Uuid;

use super::{
    dtos::{CreatePhotoRequestDto, UpdatePhotoDescriptionRequestDto},
    models::Photo,
    repository::Repository,
};
use crate::{
    domains::{
        files::{service::Service as FileService, types::UploadedFile},
        projects::authorization::authorize_project_access,
        stages::service::Service as StageService,
        tasks::service::Service as TaskService,
        users::models::UserRole,
    },
    infrastructure::storage::StorageService,
    response::{AppError, AppResult, file_response},
};

pub struct Service;

impl Service {
    pub async fn list_project_photos(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Vec<Photo>> {
        authorize_project_access(pool, project_id, user_id, role).await?;
        Repository::find_by_project_id(pool, project_id).await
    }

    pub async fn list_stage_photos(
        pool: &PgPool,
        stage_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Vec<Photo>> {
        StageService::get_stage_by_id(pool, stage_id, user_id, role).await?;
        Repository::find_by_stage_id(pool, stage_id).await
    }

    pub async fn list_task_photos(
        pool: &PgPool,
        task_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Vec<Photo>> {
        TaskService::get_task_by_id(pool, task_id, user_id, role).await?;
        Repository::find_by_task_id(pool, task_id).await
    }

    pub async fn get_photo_by_id(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Photo> {
        let photo = Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Foto não encontrada.".to_string()))?;
        authorize_project_access(pool, photo.project_id, user_id, role).await?;
        Ok(photo)
    }

    pub async fn create_photo(
        pool: &PgPool,
        dto: &CreatePhotoRequestDto,
        file: UploadedFile,
        uploaded_by: Uuid,
        role: UserRole,
    ) -> AppResult<Photo> {
        let stage = StageService::get_stage_by_id(pool, dto.stage_id, uploaded_by, role).await?;
        if stage.project_id != dto.project_id {
            return Err(AppError::BadRequest(
                "A etapa informada não pertence ao projeto.".to_string(),
            ));
        }

        if let Some(task_id) = dto.task_id {
            let task = TaskService::get_task_by_id(pool, task_id, uploaded_by, role).await?;
            if task.stage_id != dto.stage_id {
                return Err(AppError::BadRequest(
                    "A tarefa informada não pertence à etapa.".to_string(),
                ));
            }
        }

        let file = FileService::create_file(
            pool,
            "photos",
            &file.original_name,
            &file.content,
            &["image/jpeg", "image/png", "image/webp"],
            uploaded_by,
        )
        .await?;

        match Repository::create(pool, dto, file.id, uploaded_by).await {
            Ok(photo) => Ok(photo),
            Err(error) => {
                let _ = FileService::delete_file(pool, file.id).await;
                Err(error)
            }
        }
    }

    pub async fn update_description(
        pool: &PgPool,
        id: Uuid,
        dto: &UpdatePhotoDescriptionRequestDto,
    ) -> AppResult<Photo> {
        Repository::update_description(pool, id, &dto.description).await
    }

    pub async fn delete_photo(pool: &PgPool, id: Uuid) -> AppResult<()> {
        let photo = Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Foto não encontrada.".to_string()))?;
        Repository::delete(pool, id).await?;
        FileService::delete_file(pool, photo.file_id).await
    }

    pub async fn get_file(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Response> {
        let photo = Self::get_photo_by_id(pool, id, user_id, role).await?;
        let file = FileService::get_file_by_id(pool, photo.file_id).await?;
        let content = StorageService::read_file(&file.storage_key).await?;

        file_response::file(content, &file.mime_type)
    }
}

use std::collections::HashSet;

use axum::response::Response;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::{
        files::{service::Service as FileService, types::UploadedFile},
        project_images::{
            dtos::{CreateProjectImageRequestDto, ReorderProjectImagesRequestDto},
            models::ProjectImage,
            repository::Repository,
        },
        projects::service::Service as ProjectService,
    },
    infrastructure::storage::StorageService,
    response::{AppError, AppResult, file_response},
};

pub struct Service;

impl Service {
    pub async fn list_project_images(
        pool: &PgPool,
        project_id: Uuid,
    ) -> AppResult<Vec<ProjectImage>> {
        ProjectService::get_project_by_id(pool, project_id).await?;
        Repository::find_by_project_id(pool, project_id).await
    }

    pub async fn get_cover_by_project_id(
        pool: &PgPool,
        project_id: Uuid,
    ) -> AppResult<ProjectImage> {
        Repository::find_cover_by_project_id(pool, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Capa do projeto não encontrada.".to_string()))
    }

    pub async fn get_project_image_by_id(pool: &PgPool, id: Uuid) -> AppResult<ProjectImage> {
        Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Imagem do projeto não encontrada.".to_string()))
    }

    pub async fn create_project_image(
        pool: &PgPool,
        dto: &CreateProjectImageRequestDto,
        file: UploadedFile,
        uploaded_by: Uuid,
    ) -> AppResult<ProjectImage> {
        ProjectService::get_project_by_id(pool, dto.project_id).await?;

        let file = FileService::create_file(
            pool,
            "project-images",
            &file.original_name,
            &file.content,
            &["image/jpeg", "image/png", "image/webp"],
            uploaded_by,
        )
        .await?;

        let images = Repository::find_by_project_id(pool, dto.project_id).await?;

        let order = i16::try_from(images.len()).map_err(|_| {
            AppError::BadRequest("Quantidade de imagens excede o limite permitido.".to_string())
        })?;

        Repository::create(pool, dto, file.id, order).await
    }

    pub async fn reorder_project_images(
        pool: &PgPool,
        project_id: Uuid,
        dto: &ReorderProjectImagesRequestDto,
    ) -> AppResult<()> {
        let current_images = Self::list_project_images(pool, project_id).await?;

        if current_images.len() != dto.images.len() {
            return Err(AppError::BadRequest(
                "A quantidade de imagens enviada não corresponde às imagens do projeto."
                    .to_string(),
            ));
        }

        let current_ids: HashSet<Uuid> = current_images.iter().map(|image| image.id).collect();

        let requested_ids: HashSet<Uuid> = dto.images.iter().copied().collect();

        if current_ids != requested_ids {
            return Err(AppError::BadRequest(
                "As imagens enviadas não correspondem às imagens do projeto.".to_string(),
            ));
        }

        Ok(())
    }

    pub async fn delete_project_image(pool: &PgPool, id: Uuid) -> AppResult<()> {
        let image = Self::get_project_image_by_id(pool, id).await?;
        Repository::delete(pool, id, image.project_id, image.order).await
    }

    pub async fn get_file(pool: &PgPool, id: Uuid) -> AppResult<Response> {
        let project_image = Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Imagem do projeto não encontrada.".to_string()))?;

        let file = FileService::get_file_by_id(pool, project_image.file_id).await?;

        let content = StorageService::read_file(&file.storage_key).await?;

        file_response::file(content, &file.mime_type)
    }
}

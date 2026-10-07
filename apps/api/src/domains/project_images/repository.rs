use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::project_images::{
        dtos::{CreateProjectImageRequestDto, ReorderProjectImagesRequestDto},
        models::ProjectImage,
    },
    infrastructure::db::ordering::{self, OrderingTable},
    response::{AppError, AppResult},
};

pub struct Repository;

impl Repository {
    pub async fn find_by_project_id(
        pool: &PgPool,
        project_id: Uuid,
    ) -> AppResult<Vec<ProjectImage>> {
        sqlx::query_as::<_, ProjectImage>(
            r#"
                SELECT *
                FROM project_images
                WHERE project_id = $1
                ORDER BY "order" ASC, created_at DESC
            "#,
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|_| {
            AppError::Database("Falha ao buscar imagens do projeto nos dados.".to_string())
        })
    }

    pub async fn find_cover_by_project_id(
        pool: &PgPool,
        project_id: Uuid,
    ) -> AppResult<Option<ProjectImage>> {
        sqlx::query_as::<_, ProjectImage>(
            r#"
                SELECT *
                FROM project_images
                WHERE project_id = $1
                AND "order" = 0
            "#,
        )
        .bind(project_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar capa do projeto nos dados.".to_string()))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<ProjectImage>> {
        sqlx::query_as::<_, ProjectImage>(
            r#"
                SELECT *
                FROM project_images
                WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar imagem do projeto nos dados.".to_string()))
    }

    pub async fn create(
        pool: &PgPool,
        dto: &CreateProjectImageRequestDto,
        file_id: Uuid,
    ) -> AppResult<ProjectImage> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar criação de imagem do projeto".to_string())
        })?;

        let order = ordering::next_order(
            &mut transaction,
            OrderingTable::ProjectImages,
            dto.project_id,
        )
        .await?;

        let image = sqlx::query_as::<_, ProjectImage>(
            r#"
                INSERT INTO project_images (
                    id,
                    project_id,
                    file_id,
                    "order"
                )
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(dto.project_id)
        .bind(file_id)
        .bind(order)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|_| {
            AppError::Database("Falha ao criar imagem do projeto nos dados.".to_string())
        })?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao concluir criação da imagem do projeto.".to_string())
        })?;

        Ok(image)
    }

    pub async fn reorder(
        pool: &PgPool,
        project_id: Uuid,
        dto: &ReorderProjectImagesRequestDto,
    ) -> AppResult<()> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar reordenação das imagens do projeto".to_string())
        })?;

        ordering::reorder(
            &mut transaction,
            OrderingTable::ProjectImages,
            project_id,
            &dto.images,
        )
        .await?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao concluir reordenação das imagens".to_string())
        })
    }

    pub async fn delete(pool: &PgPool, id: Uuid, project_id: Uuid, order: i16) -> AppResult<()> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar deleção da imagem do projeto.".to_string())
        })?;

        let result = sqlx::query(
            r#"
                DELETE FROM project_images
                WHERE id = $1
                  AND project_id = $2
            "#,
        )
        .bind(id)
        .bind(project_id)
        .execute(&mut *transaction)
        .await
        .map_err(|_| {
            AppError::Database("Falha ao excluir imagem do projeto dos dados.".to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(
                "Imagem do projeto não encontrada.".to_string(),
            ));
        }

        ordering::shift_after_delete(
            &mut transaction,
            OrderingTable::ProjectImages,
            project_id,
            order,
        )
        .await?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao concluir deleção da imagem do projeto.".to_string())
        })?;

        Ok(())
    }
}

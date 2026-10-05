use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::project_images::{
        dtos::{CreateProjectImageRequestDto, ReorderProjectImagesRequestDto},
        models::ProjectImage,
    },
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
        order: i16,
    ) -> AppResult<ProjectImage> {
        sqlx::query_as::<_, ProjectImage>(
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
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao criar imagem do projeto nos dados.".to_string()))
    }

    pub async fn reorder(pool: &PgPool, dto: &ReorderProjectImagesRequestDto) -> AppResult<()> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar reordenação das imagens do projeto".to_string())
        })?;

        for (order, image_id) in dto.images.iter().enumerate() {
            let order = i16::try_from(order).map_err(|_| {
                AppError::BadRequest("Quantidade de imagens excede o limite permitido.".to_string())
            })?;

            sqlx::query(
                r#"
                    UPDATE project_images
                    SET "order" = $1
                    WHERE id = $2
                "#,
            )
            .bind(order)
            .bind(image_id)
            .execute(&mut *transaction)
            .await
            .map_err(|_| AppError::Database("Falha ao reordenar imagens.".to_string()))?;
        }

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao concluir reordenação das imagens".to_string())
        })
    }

    pub async fn delete(pool: &PgPool, id: Uuid, project_id: Uuid, order: i16) -> AppResult<()> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar deleção de imagem do projeto".to_string())
        })?;

        let result = sqlx::query(
            r#"
                DELETE FROM project_images
                WHERE id = $1
            "#,
        )
        .bind(id)
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

        sqlx::query(
            r#"
                UPDATE project_images
                SET "order" = "order" - 1
                WHERE project_id = $1
                AND "order" > $2
            "#,
        )
        .bind(project_id)
        .bind(order)
        .execute(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao reordenar as imagens do projeto.".to_string()))?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao concluir reordenação das imagens".to_string())
        })
    }
}

use sqlx::PgPool;
use uuid::Uuid;

use super::{dtos::CreatePhotoRequestDto, models::Photo};
use crate::response::{AppError, AppResult};

pub struct Repository;

impl Repository {
    pub async fn find_by_project_id(pool: &PgPool, project_id: Uuid) -> AppResult<Vec<Photo>> {
        sqlx::query_as::<_, Photo>(
            r#"
				SELECT *
				FROM photos
				WHERE project_id = $1
				ORDER BY created_at DESC
			"#,
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar fotos do projeto.".to_string()))
    }

    pub async fn find_by_stage_id(pool: &PgPool, stage_id: Uuid) -> AppResult<Vec<Photo>> {
        sqlx::query_as::<_, Photo>(
            r#"
				SELECT *
				FROM photos
				WHERE stage_id = $1
				ORDER BY created_at DESC
			"#,
        )
        .bind(stage_id)
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar fotos da etapa.".to_string()))
    }

    pub async fn find_by_task_id(pool: &PgPool, task_id: Uuid) -> AppResult<Vec<Photo>> {
        sqlx::query_as::<_, Photo>(
            r#"
				SELECT *
				FROM photos
				WHERE task_id = $1
				ORDER BY created_at DESC
			"#,
        )
        .bind(task_id)
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar fotos da tarefa.".to_string()))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<Photo>> {
        sqlx::query_as::<_, Photo>("SELECT * FROM photos WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao buscar foto.".to_string()))
    }

    pub async fn create(
        pool: &PgPool,
        dto: &CreatePhotoRequestDto,
        file_id: Uuid,
        uploaded_by: Uuid,
    ) -> AppResult<Photo> {
        sqlx::query_as::<_, Photo>(
            r#"
				INSERT INTO photos (
					id,
                    project_id,
                    stage_id,
                    task_id,
                    file_id,
                    description,
                    uploaded_by
				)
				VALUES ($1, $2, $3, $4, $5, $6, $7)
				RETURNING *
			"#,
        )
        .bind(Uuid::new_v4())
        .bind(dto.project_id)
        .bind(dto.stage_id)
        .bind(dto.task_id)
        .bind(file_id)
        .bind(&dto.description)
        .bind(uploaded_by)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao registrar foto.".to_string()))
    }

    pub async fn update_description(
        pool: &PgPool,
        id: Uuid,
        description: &str,
    ) -> AppResult<Photo> {
        sqlx::query_as::<_, Photo>(
            r#"
				UPDATE photos
				SET description = $1
				WHERE id = $2
				RETURNING *
			"#,
        )
        .bind(description)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao atualizar descrição da foto.".to_string()))?
        .ok_or_else(|| AppError::NotFound("Foto não encontrada.".to_string()))
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM photos WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao excluir foto.".to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Foto não encontrada.".to_string()));
        }

        Ok(())
    }
}

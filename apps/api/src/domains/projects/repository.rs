use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::projects::{
        dtos::{CreateProjectRequestDto, UpdateProjectRequestDto},
        models::Project,
    },
    response::{AppError, AppResult},
};

pub struct Repository;

impl Repository {
    pub async fn find_all(pool: &PgPool) -> AppResult<Vec<Project>> {
        sqlx::query_as::<_, Project>("SELECT * FROM projects ORDER BY created_at DESC")
            .fetch_all(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao buscar projetos nos dados.".to_string()))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<Project>> {
        sqlx::query_as::<_, Project>(
            r#"
                SELECT *
                FROM projects
                WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar projeto nos dados.".to_string()))
    }

    pub async fn create(pool: &PgPool, dto: &CreateProjectRequestDto) -> AppResult<Project> {
        sqlx::query_as::<_, Project>(
            r#"
            INSERT INTO projects (
                id,
                client_id,
                name,
                description,
                project_type,
                status,
                address,
                city,
                state,
                start_date,
                estimated_end_date,
                cover_file_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(dto.client_id)
        .bind(&dto.name)
        .bind(&dto.description)
        .bind(dto.project_type)
        .bind(dto.status)
        .bind(&dto.address)
        .bind(&dto.city)
        .bind(&dto.state)
        .bind(dto.start_date)
        .bind(dto.estimated_end_date)
        .bind(dto.cover_file_id)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao criar projeto nos dados.".to_string()))
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        dto: &UpdateProjectRequestDto,
    ) -> AppResult<Project> {
        sqlx::query_as::<_, Project>(
            r#"
            UPDATE projects
            SET name = $1,
                description = $2,
                project_type = $3,
                status = $4,
                address = $5,
                city = $6,
                state = $7,
                start_date = $8,
                estimated_end_date = $9,
                cover_file_id = $10,
                updated_at = NOW()
            WHERE id = $11
            RETURNING *
        "#,
        )
        .bind(&dto.name)
        .bind(&dto.description)
        .bind(dto.project_type)
        .bind(dto.status)
        .bind(&dto.address)
        .bind(&dto.city)
        .bind(&dto.state)
        .bind(dto.start_date)
        .bind(dto.estimated_end_date)
        .bind(dto.cover_file_id)
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao atualizar projeto nos dados.".to_string()))?
        .ok_or_else(|| AppError::NotFound("Projeto não encontrado.".to_string()))
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM projects
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao excluir projeto dos dados.".to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Projeto não encontrado.".to_string()));
        }

        Ok(())
    }
}

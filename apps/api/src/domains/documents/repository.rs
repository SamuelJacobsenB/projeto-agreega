use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::documents::{
        dtos::{CreateDocumentRequestDto, UpdateDocumentRequestDto},
        models::Document,
    },
    response::{AppError, AppResult},
};

pub struct Repository;

impl Repository {
    pub async fn find_by_stage_id(pool: &PgPool, stage_id: Uuid) -> AppResult<Vec<Document>> {
        sqlx::query_as::<_, Document>("SELECT * FROM documents WHERE stage_id = $1")
            .bind(stage_id)
            .fetch_all(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao buscar documentos nos dados.".to_string()))
    }

    pub async fn find_by_task_id(pool: &PgPool, task_id: Uuid) -> AppResult<Vec<Document>> {
        sqlx::query_as::<_, Document>("SELECT * FROM documents WHERE task_id = $1")
            .bind(task_id)
            .fetch_all(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao buscar documentos nos dados.".to_string()))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<Document>> {
        sqlx::query_as::<_, Document>("SELECT * FROM documents WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao buscar documento nos dados.".to_string()))
    }

    pub async fn create(
        pool: &PgPool,
        dto: &CreateDocumentRequestDto,
        created_by: Uuid,
    ) -> AppResult<Document> {
        sqlx::query_as::<_, Document>(
            r#"
				INSERT INTO documents (
					id,
                    project_id,
                    stage_id,
                    task_id,
                    name,
                    document_type,
                    status,
                    current_version,
                    created_by
				)
				VALUES ($1, $2, $3, $4, $5, $6, $7, 0, $8)
				RETURNING *
			"#,
        )
        .bind(Uuid::new_v4())
        .bind(dto.project_id)
        .bind(dto.stage_id)
        .bind(dto.task_id)
        .bind(&dto.name)
        .bind(dto.document_type)
        .bind(dto.status)
        .bind(created_by)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao registrar documento.".to_string()))
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        dto: &UpdateDocumentRequestDto,
    ) -> AppResult<Document> {
        sqlx::query_as::<_, Document>(
            r#"
                UPDATE documents
                SET name = COALESCE($2, name),
                    document_type = COALESCE($3, document_type),
                    status = COALESCE($4, status),
                    updated_at = NOW()
                WHERE id = $1
                RETURNING *
            "#,
        )
        .bind(id)
        .bind(&dto.name)
        .bind(dto.document_type)
        .bind(dto.status)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao atualizar documento nos dados.".to_string()))?
        .ok_or_else(|| AppError::NotFound("Documento não encontrado.".to_string()))
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM documents WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao excluir documento dos dados.".to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Documento não encontrado.".to_string()));
        }

        Ok(())
    }
}

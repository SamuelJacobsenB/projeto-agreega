use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::document_comments::{dtos::UpdateDocumentCommentRequestDto, models::DocumentComment},
    response::{AppError, AppResult},
};

pub struct Repository;

impl Repository {
    pub async fn find_by_document_version_id(
        pool: &PgPool,
        version_id: Uuid,
    ) -> AppResult<Vec<DocumentComment>> {
        sqlx::query_as::<_, DocumentComment>(
            "SELECT * FROM document_comments WHERE document_version_id = $1 ORDER BY created_at ASC",
        )
        .bind(version_id)
        .fetch_all(pool)
        .await
        .map_err(|_| {
            AppError::Database("Falha ao encontrar comentários da versão nos dados.".to_string())
        })
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<DocumentComment>> {
        sqlx::query_as::<_, DocumentComment>("SELECT * FROM document_comments WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao buscar comentário nos dados.".to_string()))
    }

    pub async fn create(
        pool: &PgPool,
        document_version_id: Uuid,
        user_id: Uuid,
        content: &str,
        page: Option<i16>,
        position_x: Option<f32>,
        position_y: Option<f32>,
    ) -> AppResult<DocumentComment> {
        sqlx::query_as::<_, DocumentComment>(
            r#"
                INSERT INTO document_comments (
                    id, document_version_id, user_id, content, page, position_x, position_y
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(document_version_id)
        .bind(user_id)
        .bind(content)
        .bind(page)
        .bind(position_x)
        .bind(position_y)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao registrar comentário.".to_string()))
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        dto: &UpdateDocumentCommentRequestDto,
    ) -> AppResult<DocumentComment> {
        sqlx::query_as::<_, DocumentComment>(
            r#"
                UPDATE document_comments
                SET content = COALESCE($2, content),
                    page = COALESCE($3, page),
                    position_x = COALESCE($4, position_x),
                    position_y = COALESCE($5, position_y),
                    updated_at = NOW()
                WHERE id = $1
                RETURNING *
            "#,
        )
        .bind(id)
        .bind(dto.content.as_deref())
        .bind(dto.page)
        .bind(dto.position_x)
        .bind(dto.position_y)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao atualizar comentário nos dados.".to_string()))?
        .ok_or_else(|| AppError::NotFound("Comentário não encontrado.".to_string()))
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM document_comments WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|_| {
                AppError::Database("Falha ao excluir comentário dos dados.".to_string())
            })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Comentário não encontrado.".to_string()));
        }

        Ok(())
    }
}

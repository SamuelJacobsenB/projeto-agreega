use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::files::models::File,
    response::{AppError, AppResult},
};

pub struct Repository;

impl Repository {
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<File>> {
        sqlx::query_as::<_, File>(
            r#"
                SELECT *
                FROM files
                WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar dados do arquivo.".to_string()))
    }

    pub async fn create(
        pool: &PgPool,
        id: Uuid,
        storage_key: &str,
        original_name: &str,
        mime_type: &str,
        size_bytes: i64,
        checksum: &str,
        uploaded_by: Uuid,
    ) -> AppResult<File> {
        sqlx::query_as::<_, File>(
            r#"
                INSERT INTO files (
                    id,
                    storage_key,
                    original_name,
                    mime_type,
                    size_bytes,
                    checksum,
                    uploaded_by
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING *
            "#,
        )
        .bind(id)
        .bind(storage_key)
        .bind(original_name)
        .bind(mime_type)
        .bind(size_bytes)
        .bind(checksum)
        .bind(uploaded_by)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao criar arquivo.".to_string()))
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM files WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao deletar arquivo.".to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Arquivo não encontrado.".to_string()));
        }

        Ok(())
    }
}

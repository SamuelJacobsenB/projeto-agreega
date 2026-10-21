use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::message_attachments::models::MessageAttachment,
    response::{AppError, AppResult},
};

pub struct Repository;

impl Repository {
    pub async fn find_by_message_id(
        pool: &PgPool,
        message_id: Uuid,
    ) -> AppResult<Vec<MessageAttachment>> {
        sqlx::query_as::<_, MessageAttachment>(
            "SELECT * FROM message_attachments WHERE message_id = $1 ORDER BY file_id",
        )
        .bind(message_id)
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar anexos da mensagem.".to_string()))
    }

    pub async fn create(
        pool: &PgPool,
        message_id: Uuid,
        file_id: Uuid,
    ) -> AppResult<MessageAttachment> {
        sqlx::query_as::<_, MessageAttachment>(
            r#"
				INSERT INTO message_attachments (message_id, file_id)
				VALUES ($1, $2)
				ON CONFLICT (message_id, file_id) DO NOTHING
				RETURNING *
			"#,
        )
        .bind(message_id)
        .bind(file_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao anexar arquivo à mensagem.".to_string()))?
        .ok_or_else(|| AppError::Conflict("Este arquivo já está anexado à mensagem.".to_string()))
    }

    pub async fn exists(pool: &PgPool, message_id: Uuid, file_id: Uuid) -> AppResult<bool> {
        sqlx::query_scalar(
			"SELECT EXISTS(SELECT 1 FROM message_attachments WHERE message_id = $1 AND file_id = $2)",
		)
		.bind(message_id)
		.bind(file_id)
		.fetch_one(pool)
		.await
		.map_err(|_| AppError::Database("Falha ao validar anexo da mensagem.".to_string()))
    }

    pub async fn delete(pool: &PgPool, message_id: Uuid, file_id: Uuid) -> AppResult<()> {
        let result =
            sqlx::query("DELETE FROM message_attachments WHERE message_id = $1 AND file_id = $2")
                .bind(message_id)
                .bind(file_id)
                .execute(pool)
                .await
                .map_err(|_| {
                    AppError::Database("Falha ao remover anexo da mensagem.".to_string())
                })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Anexo não encontrado.".to_string()));
        }

        Ok(())
    }
}

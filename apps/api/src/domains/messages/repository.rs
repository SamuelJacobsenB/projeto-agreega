use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::messages::{dtos::CreateMessageRequestDto, models::Message},
    response::{AppError, AppResult},
};

pub struct Repository;

impl Repository {
    pub async fn find_by_project_id(pool: &PgPool, project_id: Uuid) -> AppResult<Vec<Message>> {
        sqlx::query_as::<_, Message>(
            "SELECT * FROM messages WHERE project_id = $1 ORDER BY created_at ASC, id ASC",
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|_| {
            AppError::Database("Falha ao buscar mensagens do projeto nos dados.".to_string())
        })
    }

    pub async fn create(
        pool: &PgPool,
        dto: &CreateMessageRequestDto,
        sender_id: Uuid,
    ) -> AppResult<Message> {
        sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO messages (
                id,
                project_id,
                sender_id,
                content
            ) VALUES ($1, $2, $3, $4)
            RETURNING *
        "#,
        )
        .bind(Uuid::new_v4())
        .bind(dto.project_id)
        .bind(sender_id)
        .bind(&dto.content)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao salvar mensagem.".to_string()))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<Message>> {
        sqlx::query_as::<_, Message>("SELECT * FROM messages WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao buscar mensagem nos dados.".to_string()))
    }

    pub async fn update(pool: &PgPool, id: Uuid, content: &str) -> AppResult<Message> {
        sqlx::query_as::<_, Message>(
            r#"
                UPDATE messages
                SET content = $2, edited_at = NOW()
                WHERE id = $1
                RETURNING *
            "#,
        )
        .bind(id)
        .bind(content)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao atualizar mensagem nos dados.".to_string()))?
        .ok_or_else(|| AppError::NotFound("Mensagem não encontrada.".to_string()))
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM messages WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao excluir mensagem dos dados.".to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Mensagem não encontrada.".to_string()));
        }

        Ok(())
    }
}

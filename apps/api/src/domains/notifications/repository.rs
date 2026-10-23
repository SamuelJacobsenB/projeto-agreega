use sqlx::PgPool;
use uuid::Uuid;

use crate::response::{AppError, AppResult};

use super::{dtos::CreateNotificationRequestDto, models::Notification};

pub struct Repository;

impl Repository {
    pub async fn find_by_user_id(
        pool: &PgPool,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<Notification>> {
        sqlx::query_as::<_, Notification>(
            r#"
			SELECT * FROM notifications
			WHERE user_id = $1
			ORDER BY created_at DESC, id DESC
			LIMIT $2 OFFSET $3
			"#,
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar notificações.".to_string()))
    }

    pub async fn create(
        pool: &PgPool,
        dto: &CreateNotificationRequestDto,
    ) -> AppResult<Notification> {
        sqlx::query_as::<_, Notification>(
            r#"
			INSERT INTO notifications (id, user_id, title, message, notification_type)
			VALUES ($1, $2, $3, $4, $5)
			RETURNING *
			"#,
        )
        .bind(Uuid::new_v4())
        .bind(dto.user_id)
        .bind(&dto.title)
        .bind(&dto.message)
        .bind(dto.notification_type)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao criar notificação.".to_string()))
    }

    pub async fn update_read_state(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        read: bool,
    ) -> AppResult<Notification> {
        sqlx::query_as::<_, Notification>(
            r#"
			UPDATE notifications
			SET read_at = CASE
				WHEN $3 THEN COALESCE(read_at, NOW())
				ELSE NULL
			END
			WHERE id = $1 AND user_id = $2
			RETURNING *
			"#,
        )
        .bind(id)
        .bind(user_id)
        .bind(read)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao atualizar notificação.".to_string()))?
        .ok_or_else(|| AppError::NotFound("Notificação não encontrada.".to_string()))
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM notifications WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao excluir notificação.".to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(
                "Notificação não encontrada.".to_string(),
            ));
        }

        Ok(())
    }
}

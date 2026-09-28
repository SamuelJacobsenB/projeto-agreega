use sqlx::PgPool;
use uuid::Uuid;

use crate::response::{AppError, AppResult};

use super::models::User;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<User>> {
        sqlx::query_as::<_, User>(
            r#"
                SELECT *
                FROM users
                WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar usuário nos dados".to_string()))
    }
}

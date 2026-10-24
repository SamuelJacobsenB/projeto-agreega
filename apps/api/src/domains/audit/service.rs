use sqlx::PgPool;

use crate::response::{AppError, AppResult};

use super::{dtos::ListAuditLogsQueryDto, models::AuditLog, repository::Repository};

pub struct Service;

impl Service {
    pub async fn list(pool: &PgPool, query: &ListAuditLogsQueryDto) -> AppResult<Vec<AuditLog>> {
        let (before, limit) = Self::pagination(query)?;

        let entity = query.entity.as_deref().map(str::trim);
        if entity.is_some_and(str::is_empty) {
            return Err(AppError::BadRequest(
                "O filtro entity não pode estar vazio.".to_string(),
            ));
        }

        Repository::list(pool, query.user_id, entity, query.entity_id, before, limit).await
    }

    fn pagination(
        query: &ListAuditLogsQueryDto,
    ) -> AppResult<(Option<(chrono::DateTime<chrono::Utc>, uuid::Uuid)>, i64)> {
        let before = match (query.before_created_at, query.before_id) {
            (Some(created_at), Some(id)) => Some((created_at, id)),
            (None, None) => None,
            _ => {
                return Err(AppError::BadRequest(
                    "Informe before_created_at e before_id juntos.".to_string(),
                ));
            }
        };

        Ok((before, query.limit.unwrap_or(50).clamp(1, 100)))
    }
}

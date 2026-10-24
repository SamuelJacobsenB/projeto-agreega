use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{PgPool, Postgres, QueryBuilder, Transaction};
use uuid::Uuid;

use crate::response::{AppError, AppResult};

use super::models::AuditLog;

pub struct Repository;

impl Repository {
    pub async fn create_in_transaction(
        transaction: &mut Transaction<'_, Postgres>,
        user_id: Uuid,
        action: &str,
        entity: &str,
        entity_id: Uuid,
        metadata: Option<Value>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
			INSERT INTO audit_logs (id, user_id, action, entity, entity_id, metadata)
			VALUES ($1, $2, $3, $4, $5, $6)
			"#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(action)
        .bind(entity)
        .bind(entity_id)
        .bind(metadata)
        .execute(&mut **transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao registrar evento de auditoria.".to_string()))?;

        Ok(())
    }

    pub async fn list(
        pool: &PgPool,
        user_id: Option<Uuid>,
        entity: Option<&str>,
        entity_id: Option<Uuid>,
        before: Option<(DateTime<Utc>, Uuid)>,
        limit: i64,
    ) -> AppResult<Vec<AuditLog>> {
        let mut query = QueryBuilder::<Postgres>::new("SELECT * FROM audit_logs WHERE TRUE");

        if let Some(user_id) = user_id {
            query.push(" AND user_id = ").push_bind(user_id);
        }

        if let Some(entity) = entity {
            query.push(" AND entity = ").push_bind(entity);
        }

        if let Some(entity_id) = entity_id {
            query.push(" AND entity_id = ").push_bind(entity_id);
        }

        if let Some((created_at, id)) = before {
            query
                .push(" AND (created_at, id) < (")
                .push_bind(created_at)
                .push(", ")
                .push_bind(id)
                .push(")");
        }

        query
            .push(" ORDER BY created_at DESC, id DESC LIMIT ")
            .push_bind(limit);

        query
            .build_query_as::<AuditLog>()
            .fetch_all(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao consultar auditoria.".to_string()))
    }
}

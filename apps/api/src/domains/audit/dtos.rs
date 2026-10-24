use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ListAuditLogsQueryDto {
    pub user_id: Option<Uuid>,
    pub entity: Option<String>,
    pub entity_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub before_created_at: Option<DateTime<Utc>>,
    pub before_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct AuditLogResponseDto {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub entity: String,
    pub entity_id: Uuid,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
}

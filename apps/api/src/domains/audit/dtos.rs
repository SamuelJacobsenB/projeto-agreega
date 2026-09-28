use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreateAuditLogRequestDto {
    pub user_id: Option<Uuid>,
    pub action: String,
    pub entity: String,
    pub entity_id: Uuid,
    pub metadata: Option<Value>,
}

#[derive(Deserialize)]
pub struct AuditLogResponseDto {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub entity: String,
    pub entity_id: Uuid,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
}

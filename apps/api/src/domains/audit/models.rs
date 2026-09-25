use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

/// Representa um registro de uma ação realizada no sistema.
/// Conecta-se a User e à entidade afetada pela ação.
/// Usado para manter histórico, rastreabilidade e segurança das operações importantes.
pub struct AuditLog {
    pub id: Uuid,

    pub user_id: Option<Uuid>,

    pub action: String,
    pub entity: String,
    pub entity_id: Uuid,

    pub metadata: Option<Value>,

    pub created_at: DateTime<Utc>,
}

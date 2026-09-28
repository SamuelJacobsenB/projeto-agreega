use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Representa uma notificação direcionada a um usuário.
/// Conecta-se a User.
/// Usado para informar sobre eventos importantes, como novas mensagens, documentos ou aprovações.
pub struct Notification {
    pub id: Uuid,

    pub user_id: Uuid,

    pub title: String,
    pub message: String,

    pub notification_type: NotificationType,

    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "notification_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    General,
    Project,
    Document,
    Approval,
    Message,
    Task,
    Payment,
    System,
}

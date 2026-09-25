use chrono::{DateTime, Utc};
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

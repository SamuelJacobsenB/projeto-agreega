use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa uma mensagem enviada dentro de um projeto.
/// Conecta-se a Project, User e MessageAttachment.
/// Usado para comunicação entre clientes e equipe da Agreega.
pub struct Message {
    pub id: Uuid,

    pub project_id: Uuid,
    pub sender_id: Uuid,

    pub content: String,

    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
}

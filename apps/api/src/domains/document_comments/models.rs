use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa um comentário feito em um documento.
/// Conecta-se a Document e User.
/// Usado para registrar observações gerais ou posicionadas em páginas do documento.
pub struct DocumentComment {
    pub id: Uuid,

    pub document_id: Uuid,
    pub user_id: Uuid,

    pub content: String,

    pub page: Option<i32>,
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

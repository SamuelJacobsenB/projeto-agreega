use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa um comentário feito em uma versão específica de um documento.
/// Conecta-se a DocumentVersion e User.
/// Usado para registrar observações gerais ou posicionadas em páginas da versão.
#[derive(sqlx::FromRow)]
pub struct DocumentComment {
    pub id: Uuid,

    pub document_version_id: Uuid,
    pub user_id: Uuid,

    pub content: String,

    pub page: Option<i16>,
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

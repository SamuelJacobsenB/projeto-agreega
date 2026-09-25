use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa as imagens ilustrativas do projeto.
/// Conecta-se a Project, e a imagem de order = 0 é utilizada como capa
/// Usado como visualização geral do projeto
pub struct ProjectImage {
    pub id: Uuid,

    pub project_id: Uuid,
    pub file_id: Uuid,

    pub order: i16,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

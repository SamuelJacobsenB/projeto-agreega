use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa uma foto relacionada a um projeto.
/// Conecta-se a Project, File e User.
/// Usado para registrar e apresentar imagens do andamento ou execução do projeto.
pub struct Photo {
    pub id: Uuid,

    pub project_id: Uuid,
    pub file_id: Uuid,

    pub description: Option<String>,

    pub uploaded_by: Uuid,

    pub created_at: DateTime<Utc>,
}

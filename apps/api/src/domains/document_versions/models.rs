use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa uma versão específica de um documento.
/// Conecta-se a Document, File e User.
/// Usado para manter o histórico de alterações e versões dos documentos.
pub struct DocumentVersion {
    pub id: Uuid,

    pub document_id: Uuid,
    pub file_id: Uuid,

    pub version: i32,

    pub uploaded_by: Uuid,

    pub created_at: DateTime<Utc>,
}

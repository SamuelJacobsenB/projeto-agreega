use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa um documento pertencente a um projeto.
/// Conecta-se a Project, DocumentVersion, DocumentComment e DocumentApproval.
/// Usado para disponibilizar e controlar documentos relacionados ao projeto.
pub struct Document {
    pub id: Uuid,

    pub project_id: Uuid,

    pub name: String,
    pub document_type: DocumentType,
    pub status: DocumentStatus,

    pub current_version: i32,

    pub created_by: Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum DocumentType {
    Contract,
    Project,
    TechnicalDrawing,
    Report,
    Budget,
    License,
    Certificate,
    Other,
}

pub enum DocumentStatus {
    Draft,
    UnderReview,
    Approved,
    ChangesRequested,
    Archived,
}

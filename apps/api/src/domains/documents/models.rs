use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Representa um documento pertencente a um projeto.
/// Conecta-se a Project, DocumentVersion, DocumentComment e DocumentApproval.
/// Usado para disponibilizar e controlar documentos relacionados ao projeto.
pub struct Document {
    pub id: Uuid,

    pub project_id: Uuid,
    pub stage_id: Uuid,
    pub task_id: Option<Uuid>,

    pub name: String,
    pub document_type: DocumentType,
    pub status: DocumentStatus,

    pub current_version: i16,

    pub created_by: Uuid,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "document_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
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

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "document_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DocumentStatus {
    Draft,
    UnderReview,
    Approved,
    ChangesRequested,
    Archived,
}

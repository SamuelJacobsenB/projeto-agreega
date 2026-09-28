use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::models::{DocumentStatus, DocumentType};

#[derive(Serialize)]
pub struct CreateDocumentRequestDto {
    pub project_id: Uuid,
    pub stage_id: Uuid,
    pub task_id: Option<Uuid>,
    pub name: String,
    pub document_type: DocumentType,
    pub status: DocumentStatus,
    pub created_by: Uuid,
}

#[derive(Serialize)]
pub struct UpdateDocumentRequestDto {
    pub stage_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub name: Option<String>,
    pub document_type: Option<DocumentType>,
    pub status: Option<DocumentStatus>,
}

#[derive(Deserialize)]
pub struct DocumentResponseDto {
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

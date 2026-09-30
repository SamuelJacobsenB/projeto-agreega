use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::models::{DocumentStatus, DocumentType};

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateDocumentRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub project_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub stage_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub task_id: Option<Uuid>,

    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub name: String,
    pub document_type: DocumentType,
    pub status: DocumentStatus,
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub created_by: Uuid,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateDocumentRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub stage_id: Option<Uuid>,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub task_id: Option<Uuid>,

    #[validate(custom(function = "crate::validation::validate_not_blank"))]
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

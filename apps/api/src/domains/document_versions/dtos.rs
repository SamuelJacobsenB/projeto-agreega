use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateDocumentVersionRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub document_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub file_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_positive_i16"))]
    pub version: i16,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub uploaded_by: Uuid,
}

#[derive(Deserialize)]
pub struct DocumentVersionResponseDto {
    pub id: Uuid,
    pub document_id: Uuid,
    pub file_id: Uuid,
    pub version: i16,
    pub uploaded_by: Uuid,
    pub created_at: DateTime<Utc>,
}

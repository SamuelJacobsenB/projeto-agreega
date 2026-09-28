use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreateDocumentVersionRequestDto {
    pub document_id: Uuid,
    pub file_id: Uuid,
    pub version: i16,
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

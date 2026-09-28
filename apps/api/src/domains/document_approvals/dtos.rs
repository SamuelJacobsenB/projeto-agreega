use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::models::ApprovalStatus;

#[derive(Serialize)]
pub struct CreateDocumentApprovalRequestDto {
    pub document_id: Uuid,
    pub document_version_id: Uuid,
    pub comment: Option<String>,
}

#[derive(Serialize)]
pub struct ReviewDocumentApprovalRequestDto {
    pub status: ApprovalStatus,
    pub reviewed_by: Uuid,
    pub comment: Option<String>,
}

#[derive(Deserialize)]
pub struct DocumentApprovalResponseDto {
    pub id: Uuid,
    pub document_id: Uuid,
    pub document_version_id: Uuid,
    pub requested_by: Uuid,
    pub reviewed_by: Option<Uuid>,
    pub status: ApprovalStatus,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
}

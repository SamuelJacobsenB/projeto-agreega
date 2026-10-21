use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::models::ApprovalStatus;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateDocumentApprovalRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub document_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub document_version_id: Uuid,

    #[validate(length(max = 500, message = "O comentário deve ter no máximo 500 caracteres."))]
    pub comment: Option<String>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct ReviewDocumentApprovalRequestDto {
    pub status: ApprovalStatus,

    #[validate(length(max = 500, message = "O comentário deve ter no máximo 500 caracteres."))]
    pub comment: Option<String>,
}

#[derive(Deserialize, Serialize)]
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

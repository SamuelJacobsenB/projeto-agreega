use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreatePhotoRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub project_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub stage_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub task_id: Option<Uuid>,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub file_id: Uuid,
    pub description: Option<String>,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub uploaded_by: Uuid,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdatePhotoRequestDto {
    pub description: Option<String>,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub stage_id: Option<Uuid>,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub task_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct PhotoResponseDto {
    pub id: Uuid,
    pub project_id: Uuid,
    pub stage_id: Uuid,
    pub task_id: Option<Uuid>,
    pub file_id: Uuid,
    pub description: Option<String>,
    pub uploaded_by: Uuid,
    pub created_at: DateTime<Utc>,
}

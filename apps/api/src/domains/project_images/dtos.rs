use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateProjectImageRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub project_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub file_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_non_negative_i16"))]
    pub order: i16,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateProjectImageRequestDto {
    #[validate(custom(function = "crate::validation::validate_non_negative_i16"))]
    pub order: Option<i16>,
}

#[derive(Deserialize)]
pub struct ProjectImageResponseDto {
    pub id: Uuid,
    pub project_id: Uuid,
    pub file_id: Uuid,
    pub order: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

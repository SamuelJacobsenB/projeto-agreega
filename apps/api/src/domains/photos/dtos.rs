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

    #[validate(length(max = 255, message = "Descrição não pode exceder 255 caracteres."))]
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdatePhotoDescriptionRequestDto {
    #[validate(length(max = 255, message = "Descrição não pode exceder 255 caracteres."))]
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct PhotoResponseDto {
    pub id: Uuid,
    pub project_id: Uuid,
    pub stage_id: Uuid,
    pub task_id: Option<Uuid>,
    pub file_id: Uuid,
    pub description: Option<String>,
    pub url: String,
    pub uploaded_by: Uuid,
    pub created_at: DateTime<Utc>,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateProjectImageRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub project_id: Uuid,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct ReorderProjectImagesRequestDto {
    #[validate(length(
        min = 1,
        message = "Para reordenar as imagens do projeto todas as imagens deve, ser listadas."
    ))]
    pub images: Vec<Uuid>,
}

#[derive(Deserialize, Serialize)]
pub struct ProjectImageResponseDto {
    pub id: Uuid,
    pub project_id: Uuid,
    pub file_id: Uuid,
    pub url: String,
    pub order: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

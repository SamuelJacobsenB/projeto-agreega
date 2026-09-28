use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreateProjectImageRequestDto {
    pub project_id: Uuid,
    pub file_id: Uuid,
    pub order: i16,
}

#[derive(Serialize)]
pub struct UpdateProjectImageRequestDto {
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

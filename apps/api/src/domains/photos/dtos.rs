use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreatePhotoRequestDto {
    pub project_id: Uuid,
    pub stage_id: Uuid,
    pub task_id: Option<Uuid>,
    pub file_id: Uuid,
    pub description: Option<String>,
    pub uploaded_by: Uuid,
}

#[derive(Serialize)]
pub struct UpdatePhotoRequestDto {
    pub description: Option<String>,
    pub stage_id: Option<Uuid>,
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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreateMessageRequestDto {
    pub project_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
}

#[derive(Serialize)]
pub struct UpdateMessageRequestDto {
    pub content: Option<String>,
}

#[derive(Deserialize)]
pub struct MessageResponseDto {
    pub id: Uuid,
    pub project_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
}

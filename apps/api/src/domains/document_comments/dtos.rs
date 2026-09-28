use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreateDocumentCommentRequestDto {
    pub document_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub page: Option<i32>,
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,
}

#[derive(Serialize)]
pub struct UpdateDocumentCommentRequestDto {
    pub content: Option<String>,
    pub page: Option<i32>,
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,
}

#[derive(Deserialize)]
pub struct DocumentCommentResponseDto {
    pub id: Uuid,
    pub document_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub page: Option<i32>,
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

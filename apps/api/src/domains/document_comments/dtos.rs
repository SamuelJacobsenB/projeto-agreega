use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateDocumentCommentRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub document_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub user_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub content: String,
    pub page: Option<i32>,
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateDocumentCommentRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
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

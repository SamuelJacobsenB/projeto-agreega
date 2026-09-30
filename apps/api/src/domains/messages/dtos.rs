use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateMessageRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub project_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub sender_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub content: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateMessageRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
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

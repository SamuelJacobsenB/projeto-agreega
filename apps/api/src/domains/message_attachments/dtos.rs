use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateMessageAttachmentRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub message_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub file_id: Uuid,
}

#[derive(Deserialize)]
pub struct MessageAttachmentResponseDto {
    pub message_id: Uuid,
    pub file_id: Uuid,
}

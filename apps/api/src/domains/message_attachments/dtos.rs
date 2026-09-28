use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreateMessageAttachmentRequestDto {
    pub message_id: Uuid,
    pub file_id: Uuid,
}

#[derive(Deserialize)]
pub struct MessageAttachmentResponseDto {
    pub message_id: Uuid,
    pub file_id: Uuid,
}

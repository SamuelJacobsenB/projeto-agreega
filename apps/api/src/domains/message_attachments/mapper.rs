use super::{dtos::MessageAttachmentResponseDto, models::MessageAttachment};

impl From<MessageAttachment> for MessageAttachmentResponseDto {
    fn from(message_attachment: MessageAttachment) -> Self {
        Self {
            message_id: message_attachment.message_id,
            file_id: message_attachment.file_id,
        }
    }
}

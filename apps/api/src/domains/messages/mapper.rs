use super::{dtos::MessageResponseDto, models::Message};

impl From<Message> for MessageResponseDto {
    fn from(message: Message) -> Self {
        MessageResponseDto {
            id: message.id,
            project_id: message.project_id,
            sender_id: message.sender_id,
            content: message.content,
            created_at: message.created_at,
            edited_at: message.edited_at,
        }
    }
}

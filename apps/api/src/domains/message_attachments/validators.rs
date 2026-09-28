use crate::response::{AppError, AppResult};
use uuid::Uuid;

use super::dtos::CreateMessageAttachmentRequestDto;

pub fn validate_create_message_attachment_request(
    dto: &CreateMessageAttachmentRequestDto,
) -> AppResult<()> {
    if dto.message_id == Uuid::nil() {
        return Err(AppError::Validation("Mensagem obrigatória.".to_string()));
    }
    if dto.file_id == Uuid::nil() {
        return Err(AppError::Validation("Arquivo obrigatório.".to_string()));
    }
    Ok(())
}

use crate::response::{AppError, AppResult};
use uuid::Uuid;

use super::dtos::{CreateMessageRequestDto, UpdateMessageRequestDto};

pub fn validate_create_message_request(dto: &CreateMessageRequestDto) -> AppResult<()> {
    if dto.project_id == Uuid::nil() {
        return Err(AppError::Validation("Projeto obrigatório.".to_string()));
    }
    if dto.sender_id == Uuid::nil() {
        return Err(AppError::Validation("Remetente obrigatório.".to_string()));
    }
    if dto.content.trim().is_empty() {
        return Err(AppError::Validation("Mensagem obrigatória.".to_string()));
    }
    Ok(())
}

pub fn validate_update_message_request(dto: &UpdateMessageRequestDto) -> AppResult<()> {
    if let Some(content) = &dto.content {
        if content.trim().is_empty() {
            return Err(AppError::Validation(
                "Mensagem não pode ficar vazia.".to_string(),
            ));
        }
    }
    Ok(())
}

use super::dtos::{CreateNotificationRequestDto, UpdateNotificationRequestDto};
use crate::response::{AppError, AppResult};

pub fn validate_create_notification_request(dto: &CreateNotificationRequestDto) -> AppResult<()> {
    if dto.user_id == uuid::Uuid::nil() {
        return Err(AppError::Validation("Usuário obrigatório.".to_string()));
    }
    if dto.title.trim().is_empty() {
        return Err(AppError::Validation("Título obrigatório.".to_string()));
    }
    if dto.message.trim().is_empty() {
        return Err(AppError::Validation("Mensagem obrigatória.".to_string()));
    }
    Ok(())
}

pub fn validate_update_notification_request(dto: &UpdateNotificationRequestDto) -> AppResult<()> {
    if !dto.read {
        return Ok(());
    }
    Ok(())
}

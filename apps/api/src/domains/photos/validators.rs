use crate::response::{AppError, AppResult};
use uuid::Uuid;

use super::dtos::{CreatePhotoRequestDto, UpdatePhotoRequestDto};

pub fn validate_create_photo_request(dto: &CreatePhotoRequestDto) -> AppResult<()> {
    if dto.project_id == Uuid::nil() {
        return Err(AppError::Validation("Projeto obrigatório.".to_string()));
    }
    if dto.stage_id == Uuid::nil() {
        return Err(AppError::Validation("Etapa obrigatória.".to_string()));
    }
    if dto.file_id == Uuid::nil() {
        return Err(AppError::Validation("Arquivo obrigatório.".to_string()));
    }
    if dto.uploaded_by == Uuid::nil() {
        return Err(AppError::Validation("Remetente obrigatório.".to_string()));
    }
    Ok(())
}

pub fn validate_update_photo_request(dto: &UpdatePhotoRequestDto) -> AppResult<()> {
    if let Some(stage_id) = dto.stage_id {
        if stage_id == Uuid::nil() {
            return Err(AppError::Validation("Etapa inválida.".to_string()));
        }
    }
    if let Some(task_id) = dto.task_id {
        if task_id == Uuid::nil() {
            return Err(AppError::Validation("Tarefa inválida.".to_string()));
        }
    }
    Ok(())
}

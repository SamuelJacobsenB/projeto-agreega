use crate::response::{AppError, AppResult};
use uuid::Uuid;

use super::dtos::{CreateProjectImageRequestDto, UpdateProjectImageRequestDto};

pub fn validate_create_project_image_request(dto: &CreateProjectImageRequestDto) -> AppResult<()> {
    if dto.project_id == Uuid::nil() {
        return Err(AppError::Validation("Projeto obrigatório.".to_string()));
    }
    if dto.file_id == Uuid::nil() {
        return Err(AppError::Validation("Arquivo obrigatório.".to_string()));
    }
    Ok(())
}

pub fn validate_update_project_image_request(dto: &UpdateProjectImageRequestDto) -> AppResult<()> {
    if let Some(order) = dto.order {
        if order < 0 {
            return Err(AppError::Validation("Ordem inválida.".to_string()));
        }
    }
    Ok(())
}

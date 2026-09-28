use crate::response::{AppError, AppResult};
use uuid::Uuid;

use super::dtos::{CreateTaskRequestDto, UpdateTaskRequestDto};

pub fn validate_create_task_request(dto: &CreateTaskRequestDto) -> AppResult<()> {
    if dto.stage_id == Uuid::nil() {
        return Err(AppError::Validation("Etapa obrigatória.".to_string()));
    }
    if dto.title.trim().is_empty() {
        return Err(AppError::Validation("Título obrigatório.".to_string()));
    }
    if dto.weight <= rust_decimal::Decimal::ZERO {
        return Err(AppError::Validation(
            "Peso deve ser maior que zero.".to_string(),
        ));
    }
    if dto.progress < rust_decimal::Decimal::ZERO
        || dto.progress > rust_decimal::Decimal::new(100, 0)
    {
        return Err(AppError::Validation(
            "Progresso deve estar entre 0 e 100.".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_update_task_request(dto: &UpdateTaskRequestDto) -> AppResult<()> {
    if let Some(title) = &dto.title {
        if title.trim().is_empty() {
            return Err(AppError::Validation(
                "Título não pode ficar vazio.".to_string(),
            ));
        }
    }
    if let Some(weight) = &dto.weight {
        if *weight <= rust_decimal::Decimal::ZERO {
            return Err(AppError::Validation(
                "Peso deve ser maior que zero.".to_string(),
            ));
        }
    }
    if let Some(progress) = &dto.progress {
        if *progress < rust_decimal::Decimal::ZERO || *progress > rust_decimal::Decimal::new(100, 0)
        {
            return Err(AppError::Validation(
                "Progresso deve estar entre 0 e 100.".to_string(),
            ));
        }
    }
    Ok(())
}

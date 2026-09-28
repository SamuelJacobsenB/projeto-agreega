use super::dtos::{CreateStageRequestDto, UpdateStageRequestDto};
use crate::response::{AppError, AppResult};

pub fn validate_create_stage_request(dto: &CreateStageRequestDto) -> AppResult<()> {
    if dto.name.trim().is_empty() {
        return Err(AppError::Validation("Nome obrigatório.".to_string()));
    }
    if dto.weight <= rust_decimal::Decimal::ZERO {
        return Err(AppError::Validation(
            "Peso deve ser maior que zero.".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_update_stage_request(dto: &UpdateStageRequestDto) -> AppResult<()> {
    if let Some(name) = &dto.name {
        if name.trim().is_empty() {
            return Err(AppError::Validation(
                "Nome não pode ficar vazio.".to_string(),
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
    Ok(())
}

use crate::response::{AppError, AppResult};
use uuid::Uuid;

use super::dtos::{CreatePaymentRequestDto, UpdatePaymentRequestDto};

pub fn validate_create_payment_request(dto: &CreatePaymentRequestDto) -> AppResult<()> {
    if dto.project_id == Uuid::nil() {
        return Err(AppError::Validation("Projeto obrigatório.".to_string()));
    }
    if dto.description.trim().is_empty() {
        return Err(AppError::Validation("Descrição obrigatória.".to_string()));
    }
    if dto.amount <= rust_decimal::Decimal::ZERO {
        return Err(AppError::Validation(
            "Valor deve ser maior que zero.".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_update_payment_request(dto: &UpdatePaymentRequestDto) -> AppResult<()> {
    if let Some(description) = &dto.description {
        if description.trim().is_empty() {
            return Err(AppError::Validation(
                "Descrição não pode ficar vazia.".to_string(),
            ));
        }
    }
    if let Some(amount) = &dto.amount {
        if *amount <= rust_decimal::Decimal::ZERO {
            return Err(AppError::Validation(
                "Valor deve ser maior que zero.".to_string(),
            ));
        }
    }
    Ok(())
}

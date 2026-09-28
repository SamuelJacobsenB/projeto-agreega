use super::dtos::{CreateClientRequestDto, UpdateClientRequestDto};
use crate::response::{AppError, AppResult};

pub fn validate_create_client_request(dto: &CreateClientRequestDto) -> AppResult<()> {
    if let Some(company_name) = &dto.company_name {
        if company_name.trim().is_empty() {
            return Err(AppError::Validation(
                "Nome da empresa não pode ficar vazio.".to_string(),
            ));
        }
    }
    if let Some(document) = &dto.document {
        if document.trim().is_empty() {
            return Err(AppError::Validation(
                "Documento não pode ficar vazio.".to_string(),
            ));
        }
    }
    if let Some(email) = &dto.email {
        if !email.trim().is_empty() && !email.contains('@') {
            return Err(AppError::Validation("E-mail inválido.".to_string()));
        }
    }
    Ok(())
}

pub fn validate_update_client_request(dto: &UpdateClientRequestDto) -> AppResult<()> {
    if let Some(company_name) = &dto.company_name {
        if company_name.trim().is_empty() {
            return Err(AppError::Validation(
                "Nome da empresa não pode ficar vazio.".to_string(),
            ));
        }
    }
    if let Some(document) = &dto.document {
        if document.trim().is_empty() {
            return Err(AppError::Validation(
                "Documento não pode ficar vazio.".to_string(),
            ));
        }
    }
    if let Some(email) = &dto.email {
        if !email.trim().is_empty() && !email.contains('@') {
            return Err(AppError::Validation("E-mail inválido.".to_string()));
        }
    }
    Ok(())
}

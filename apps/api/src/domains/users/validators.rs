use crate::response::{AppError, AppResult};
use uuid::Uuid;

use super::dtos::{CreateUserRequestDto, UpdateUserRequestDto};

fn is_valid_email(value: &str) -> bool {
    let value = value.trim();
    let mut parts = value.split('@');
    let local = parts.next();
    let domain = parts.next();
    let rest = parts.next();

    matches!(local, Some(local) if !local.is_empty())
        && matches!(domain, Some(domain) if !domain.is_empty() && domain.contains('.'))
        && rest.is_none()
}

pub fn validate_create_user_request(dto: &CreateUserRequestDto) -> AppResult<()> {
    if dto.name.trim().is_empty() {
        return Err(AppError::Validation("Nome obrigatório.".to_string()));
    }
    if !is_valid_email(&dto.email) {
        return Err(AppError::Validation("E-mail inválido.".to_string()));
    }
    if dto.password.trim().len() < 8 {
        return Err(AppError::Validation(
            "A senha deve ter pelo menos 8 caracteres.".to_string(),
        ));
    }
    if let Some(client_id) = dto.client_id {
        if client_id == Uuid::nil() {
            return Err(AppError::Validation(
                "Dados do cliente inválidos.".to_string(),
            ));
        }
    }
    if let Some(avatar_file_id) = dto.avatar_file_id {
        if avatar_file_id == Uuid::nil() {
            return Err(AppError::Validation(
                "Dados da imagem inválidos.".to_string(),
            ));
        }
    }
    Ok(())
}

pub fn validate_update_user_request(dto: &UpdateUserRequestDto) -> AppResult<()> {
    if let Some(name) = &dto.name {
        if name.trim().is_empty() {
            return Err(AppError::Validation(
                "Nome não pode ficar vazio.".to_string(),
            ));
        }
    }
    if let Some(email) = &dto.email {
        if !email.trim().is_empty() && !is_valid_email(email) {
            return Err(AppError::Validation("E-mail inválido.".to_string()));
        }
    }
    if let Some(client_id) = dto.client_id {
        if client_id == Uuid::nil() {
            return Err(AppError::Validation(
                "Dados do cliente inválidos.".to_string(),
            ));
        }
    }
    if let Some(avatar_file_id) = dto.avatar_file_id {
        if avatar_file_id == Uuid::nil() {
            return Err(AppError::Validation(
                "Dados da imagem inválidos.".to_string(),
            ));
        }
    }
    Ok(())
}

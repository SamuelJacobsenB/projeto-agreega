use crate::response::{AppError, AppResult};
use chrono::Utc;
use uuid::Uuid;

use super::{
    dtos::{
        AcceptInvitationRequestDto, CreateInvitationRequestDto, LoginRequestDto,
        RequestPasswordResetRequestDto, ResetPasswordRequestDto,
    },
    models::Invitation,
};

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

pub fn validate_login_request(dto: &LoginRequestDto) -> AppResult<()> {
    if dto.email.trim().is_empty() {
        return Err(AppError::Validation("E-mail obrigatório.".to_string()));
    }
    if !is_valid_email(&dto.email) {
        return Err(AppError::Validation("E-mail inválido.".to_string()));
    }
    if dto.password.trim().len() < 8 {
        return Err(AppError::Validation(
            "A senha deve ter pelo menos 8 caracteres.".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_create_invitation_request(dto: &CreateInvitationRequestDto) -> AppResult<()> {
    if dto.email.trim().is_empty() {
        return Err(AppError::Validation("E-mail obrigatório.".to_string()));
    }
    if !is_valid_email(&dto.email) {
        return Err(AppError::Validation("E-mail inválido.".to_string()));
    }
    if let Some(client_id) = dto.client_id {
        if client_id == Uuid::nil() {
            return Err(AppError::Validation(
                "Dados do cliente inválidos.".to_string(),
            ));
        }
    }
    Ok(())
}

pub fn validate_accept_invitation_request(dto: &AcceptInvitationRequestDto) -> AppResult<()> {
    if dto.token.trim().is_empty() {
        return Err(AppError::Validation("Token obrigatório.".to_string()));
    }
    if dto.name.trim().is_empty() {
        return Err(AppError::Validation("Nome obrigatório.".to_string()));
    }
    if dto.password.trim().len() < 8 {
        return Err(AppError::Validation(
            "A senha deve ter pelo menos 8 caracteres.".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_password_reset_request(dto: &RequestPasswordResetRequestDto) -> AppResult<()> {
    if dto.email.trim().is_empty() {
        return Err(AppError::Validation("E-mail obrigatório.".to_string()));
    }
    if !is_valid_email(&dto.email) {
        return Err(AppError::Validation("E-mail inválido.".to_string()));
    }
    Ok(())
}

pub fn validate_reset_password_request(dto: &ResetPasswordRequestDto) -> AppResult<()> {
    if dto.token.trim().is_empty() {
        return Err(AppError::Validation("Token obrigatório.".to_string()));
    }
    if dto.password.trim().len() < 8 {
        return Err(AppError::Validation(
            "A senha deve ter pelo menos 8 caracteres.".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_invitation(invitation: &Invitation) -> AppResult<()> {
    if invitation.email.trim().is_empty() {
        return Err(AppError::Validation("E-mail obrigatório.".to_string()));
    }
    if invitation.token_hash.trim().is_empty() {
        return Err(AppError::Validation(
            "Dados de convite inválidos.".to_string(),
        ));
    }
    if invitation.expires_at <= Utc::now() {
        return Err(AppError::Validation("Convite expirado.".to_string()));
    }
    Ok(())
}

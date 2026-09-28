use crate::response::{AppError, AppResult};
use uuid::Uuid;

use super::dtos::CreateAuditLogRequestDto;

pub fn validate_create_audit_log_request(dto: &CreateAuditLogRequestDto) -> AppResult<()> {
    if dto.action.trim().is_empty() {
        return Err(AppError::Validation("Ação obrigatória.".to_string()));
    }
    if dto.entity.trim().is_empty() {
        return Err(AppError::Validation("Entidade obrigatória.".to_string()));
    }
    if dto.entity_id == Uuid::nil() {
        return Err(AppError::Validation(
            "Identificador da entidade obrigatório.".to_string(),
        ));
    }
    Ok(())
}

use crate::response::{AppError, AppResult};
use uuid::Uuid;

use super::dtos::{CreateDocumentApprovalRequestDto, ReviewDocumentApprovalRequestDto};

pub fn validate_create_document_approval_request(
    dto: &CreateDocumentApprovalRequestDto,
) -> AppResult<()> {
    if dto.document_id == Uuid::nil() {
        return Err(AppError::Validation("Documento obrigatório.".to_string()));
    }
    if dto.document_version_id == Uuid::nil() {
        return Err(AppError::Validation(
            "Versão do documento obrigatória.".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_review_document_approval_request(
    dto: &ReviewDocumentApprovalRequestDto,
) -> AppResult<()> {
    if dto.reviewed_by == Uuid::nil() {
        return Err(AppError::Validation(
            "Usuário responsável obrigatório.".to_string(),
        ));
    }
    Ok(())
}

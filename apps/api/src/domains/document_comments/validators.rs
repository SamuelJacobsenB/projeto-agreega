use crate::response::{AppError, AppResult};
use uuid::Uuid;

use super::dtos::{CreateDocumentCommentRequestDto, UpdateDocumentCommentRequestDto};

pub fn validate_create_document_comment_request(
    dto: &CreateDocumentCommentRequestDto,
) -> AppResult<()> {
    if dto.document_id == Uuid::nil() {
        return Err(AppError::Validation("Documento obrigatório.".to_string()));
    }
    if dto.user_id == Uuid::nil() {
        return Err(AppError::Validation("Usuário obrigatório.".to_string()));
    }
    if dto.content.trim().is_empty() {
        return Err(AppError::Validation("Mensagem obrigatória.".to_string()));
    }
    Ok(())
}

pub fn validate_update_document_comment_request(
    dto: &UpdateDocumentCommentRequestDto,
) -> AppResult<()> {
    if let Some(content) = &dto.content {
        if content.trim().is_empty() {
            return Err(AppError::Validation(
                "Mensagem não pode ficar vazia.".to_string(),
            ));
        }
    }
    Ok(())
}

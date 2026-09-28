use crate::response::{AppError, AppResult};
use uuid::Uuid;

use super::dtos::CreateDocumentVersionRequestDto;

pub fn validate_create_document_version_request(
    dto: &CreateDocumentVersionRequestDto,
) -> AppResult<()> {
    if dto.document_id == Uuid::nil() {
        return Err(AppError::Validation("Documento obrigatório.".to_string()));
    }
    if dto.file_id == Uuid::nil() {
        return Err(AppError::Validation("Arquivo obrigatório.".to_string()));
    }
    if dto.version <= 0 {
        return Err(AppError::Validation("Versão inválida.".to_string()));
    }
    Ok(())
}

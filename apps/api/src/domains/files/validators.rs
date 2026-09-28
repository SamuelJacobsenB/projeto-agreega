use crate::response::{AppError, AppResult};
use uuid::Uuid;

use super::dtos::CreateFileRequestDto;

pub fn validate_create_file_request(dto: &CreateFileRequestDto) -> AppResult<()> {
    if dto.storage_key.trim().is_empty() {
        return Err(AppError::Validation("Arquivo inválido.".to_string()));
    }
    if dto.original_name.trim().is_empty() {
        return Err(AppError::Validation(
            "Nome do arquivo obrigatório.".to_string(),
        ));
    }
    if dto.mime_type.trim().is_empty() {
        return Err(AppError::Validation(
            "Tipo do arquivo obrigatório.".to_string(),
        ));
    }
    if dto.size_bytes <= 0 {
        return Err(AppError::Validation(
            "Tamanho do arquivo inválido.".to_string(),
        ));
    }
    if dto.uploaded_by == Uuid::nil() {
        return Err(AppError::Validation("Remetente obrigatório.".to_string()));
    }
    Ok(())
}

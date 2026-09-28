use crate::response::{AppError, AppResult};
use uuid::Uuid;

use super::dtos::{CreateDocumentRequestDto, UpdateDocumentRequestDto};

pub fn validate_create_document_request(dto: &CreateDocumentRequestDto) -> AppResult<()> {
    if dto.project_id == Uuid::nil() {
        return Err(AppError::Validation("Projeto obrigatório.".to_string()));
    }
    if dto.stage_id == Uuid::nil() {
        return Err(AppError::Validation("Etapa obrigatória.".to_string()));
    }
    if dto.name.trim().is_empty() {
        return Err(AppError::Validation("Nome obrigatório.".to_string()));
    }
    if dto.created_by == Uuid::nil() {
        return Err(AppError::Validation(
            "Responsável pela criação obrigatório.".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_update_document_request(dto: &UpdateDocumentRequestDto) -> AppResult<()> {
    if let Some(stage_id) = dto.stage_id {
        if stage_id == Uuid::nil() {
            return Err(AppError::Validation("Etapa inválida.".to_string()));
        }
    }
    if let Some(task_id) = dto.task_id {
        if task_id == Uuid::nil() {
            return Err(AppError::Validation("Tarefa inválida.".to_string()));
        }
    }
    if let Some(name) = &dto.name {
        if name.trim().is_empty() {
            return Err(AppError::Validation(
                "Nome não pode ficar vazio.".to_string(),
            ));
        }
    }
    Ok(())
}

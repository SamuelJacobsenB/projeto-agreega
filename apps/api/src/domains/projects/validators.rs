use super::dtos::{CreateProjectRequestDto, UpdateProjectRequestDto};
use crate::response::{AppError, AppResult};

pub fn validate_create_project_request(dto: &CreateProjectRequestDto) -> AppResult<()> {
    if dto.name.trim().is_empty() {
        return Err(AppError::Validation("Nome obrigatório.".to_string()));
    }
    if dto.city.trim().is_empty() {
        return Err(AppError::Validation("Cidade obrigatória.".to_string()));
    }
    if dto.state.trim().is_empty() {
        return Err(AppError::Validation("Estado obrigatório.".to_string()));
    }
    Ok(())
}

pub fn validate_update_project_request(dto: &UpdateProjectRequestDto) -> AppResult<()> {
    if let Some(name) = &dto.name {
        if name.trim().is_empty() {
            return Err(AppError::Validation(
                "Nome não pode ficar vazio.".to_string(),
            ));
        }
    }
    if let Some(city) = &dto.city {
        if city.trim().is_empty() {
            return Err(AppError::Validation(
                "Cidade não pode ficar vazia.".to_string(),
            ));
        }
    }
    if let Some(state) = &dto.state {
        if state.trim().is_empty() {
            return Err(AppError::Validation(
                "Estado não pode ficar vazio.".to_string(),
            ));
        }
    }
    Ok(())
}

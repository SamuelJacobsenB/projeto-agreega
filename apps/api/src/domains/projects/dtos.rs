use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::models::{ProjectStatus, ProjectType};

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateProjectRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub client_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub name: String,
    #[validate(length(max = 4000, message = "Descrição não pode exceder 4000 caracteres."))]
    pub description: Option<String>,
    pub project_type: ProjectType,
    pub status: ProjectStatus,

    pub address: Option<String>,
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub city: String,
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub state: String,

    pub start_date: Option<NaiveDate>,
    pub estimated_end_date: Option<NaiveDate>,
    pub cover_file_id: Option<Uuid>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateProjectRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub name: Option<String>,
    #[validate(length(max = 4000, message = "Descrição não pode exceder 4000 caracteres."))]
    pub description: Option<String>,
    pub project_type: Option<ProjectType>,
    pub status: Option<ProjectStatus>,

    pub address: Option<String>,
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub city: Option<String>,
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub state: Option<String>,

    pub start_date: Option<NaiveDate>,
    pub estimated_end_date: Option<NaiveDate>,
    pub cover_file_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct ProjectResponseDto {
    pub id: Uuid,
    pub client_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub project_type: ProjectType,
    pub status: ProjectStatus,
    pub address: Option<String>,
    pub city: String,
    pub state: String,
    pub start_date: Option<NaiveDate>,
    pub estimated_end_date: Option<NaiveDate>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cover_file_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

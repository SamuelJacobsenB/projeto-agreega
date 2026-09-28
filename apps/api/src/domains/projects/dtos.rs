use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::models::{ProjectStatus, ProjectType};

#[derive(Serialize)]
pub struct CreateProjectRequestDto {
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
    pub cover_file_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct UpdateProjectRequestDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub project_type: Option<ProjectType>,
    pub status: Option<ProjectStatus>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub estimated_end_date: Option<NaiveDate>,
    pub cover_file_id: Option<Uuid>,
}

#[derive(Deserialize)]
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

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Representa um projeto administrado pela Agreega.
/// Conecta-se a Client, Stage, Document, Message, Photo e Payment.
/// Usado como entidade central do portal para acompanhar e gerenciar cada projeto.
pub struct Project {
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

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "project_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ProjectType {
    Architecture,
    Engineering,
    ArchitectureAndEngineering,
    Consulting,
    Other,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "project_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Planning,
    InProgress,
    Paused,
    Completed,
    Cancelled,
}

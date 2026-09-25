use chrono::{DateTime, NaiveDate, Utc};
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

pub enum ProjectType {
    Architecture,
    Engineering,
    ArchitectureAndEngineering,
    Consulting,
    Other,
}

pub enum ProjectStatus {
    Planning,
    InProgress,
    Paused,
    Completed,
    Cancelled,
}

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Representa uma etapa dentro de um projeto.
/// Conecta-se a Project e Task.
/// Usado para organizar o projeto em fases e calcular seu progresso.
pub struct Stage {
    pub id: Uuid,

    pub project_id: Uuid,

    pub name: String,
    pub description: Option<String>,

    pub weight: Decimal,
    pub order: i16,

    pub status: StageStatus,

    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[sqlx(type_name = "stage_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum StageStatus {
    Pending,
    InProgress,
    Completed,
    Blocked,
    Cancelled,
}

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

/// Representa uma tarefa pertencente a uma etapa.
/// Conecta-se a Stage e User.
/// Usado para controlar atividades, responsáveis, prazos e progresso das etapas.
pub struct Task {
    pub id: Uuid,

    pub stage_id: Uuid,

    pub title: String,
    pub description: Option<String>,

    pub weight: Decimal,
    pub progress: Decimal,
    pub order: i32,

    pub status: TaskStatus,

    pub assigned_to: Option<Uuid>,

    pub due_date: Option<NaiveDate>,
    pub completed_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Blocked,
    Cancelled,
}

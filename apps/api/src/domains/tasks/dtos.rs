use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::models::TaskStatus;

#[derive(Serialize)]
pub struct CreateTaskRequestDto {
    pub stage_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub weight: Decimal,
    pub progress: Decimal,
    pub order: i16,
    pub status: TaskStatus,
    pub assigned_to: Option<Uuid>,
    pub due_date: Option<NaiveDate>,
}

#[derive(Serialize)]
pub struct UpdateTaskRequestDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub weight: Option<Decimal>,
    pub progress: Option<Decimal>,
    pub order: Option<i16>,
    pub status: Option<TaskStatus>,
    pub assigned_to: Option<Uuid>,
    pub due_date: Option<NaiveDate>,
}

#[derive(Deserialize)]
pub struct TaskResponseDto {
    pub id: Uuid,
    pub stage_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub weight: Decimal,
    pub progress: Decimal,
    pub order: i16,
    pub status: TaskStatus,
    pub assigned_to: Option<Uuid>,
    pub due_date: Option<NaiveDate>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

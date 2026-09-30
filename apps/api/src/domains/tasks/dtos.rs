use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::models::TaskStatus;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateTaskRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub stage_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub title: String,
    pub description: Option<String>,
    #[validate(custom(function = "crate::validation::validate_positive_decimal"))]
    pub weight: Decimal,

    #[validate(custom(function = "crate::validation::validate_percentage"))]
    pub progress: Decimal,
    pub order: i16,
    pub status: TaskStatus,
    pub assigned_to: Option<Uuid>,
    pub due_date: Option<NaiveDate>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateTaskRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub title: Option<String>,
    pub description: Option<String>,

    #[validate(custom(function = "crate::validation::validate_positive_decimal"))]
    pub weight: Option<Decimal>,

    #[validate(custom(function = "crate::validation::validate_percentage"))]
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

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::models::StageStatus;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateStageRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub project_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub name: String,
    #[validate(length(max = 1000, message = "Descrição não pode exceder 1000 caracteres."))]
    pub description: Option<String>,
    #[validate(custom(function = "crate::validation::validate_positive_decimal"))]
    pub weight: Decimal,
    pub status: StageStatus,

    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateStageRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub name: Option<String>,
    #[validate(length(max = 1000, message = "Descrição não pode exceder 1000 caracteres."))]
    pub description: Option<String>,
    #[validate(custom(function = "crate::validation::validate_positive_decimal"))]
    pub weight: Option<Decimal>,
    pub status: Option<StageStatus>,

    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct ReorderStagesRequestDto {
    #[validate(length(
        min = 1,
        message = "Para reordenar os estágios do projeto todos os estágios devem ser listadas."
    ))]
    pub stages: Vec<Uuid>,
}

#[derive(Serialize, Deserialize)]
pub struct StageResponseDto {
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

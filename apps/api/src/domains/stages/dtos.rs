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
    pub description: Option<String>,
    #[validate(custom(function = "crate::validation::validate_positive_decimal"))]
    pub weight: Decimal,
    pub order: i16,
    pub status: StageStatus,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateStageRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub name: Option<String>,
    pub description: Option<String>,

    #[validate(custom(function = "crate::validation::validate_positive_decimal"))]
    pub weight: Option<Decimal>,
    pub order: Option<i16>,
    pub status: Option<StageStatus>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Deserialize)]
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

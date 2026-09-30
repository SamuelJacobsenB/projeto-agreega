use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::models::PaymentStatus;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreatePaymentRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub project_id: Uuid,

    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub description: String,

    #[validate(custom(function = "crate::validation::validate_positive_decimal"))]
    pub amount: Decimal,
    pub due_date: Option<NaiveDate>,
    pub status: PaymentStatus,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdatePaymentRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub description: Option<String>,

    #[validate(custom(function = "crate::validation::validate_positive_decimal"))]
    pub amount: Option<Decimal>,
    pub due_date: Option<NaiveDate>,
    pub status: Option<PaymentStatus>,
}

#[derive(Deserialize)]
pub struct PaymentResponseDto {
    pub id: Uuid,
    pub project_id: Uuid,
    pub description: String,
    pub amount: Decimal,
    pub due_date: Option<NaiveDate>,
    pub paid_at: Option<DateTime<Utc>>,
    pub status: PaymentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

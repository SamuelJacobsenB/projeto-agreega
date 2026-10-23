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

    #[validate(length(max = 255))]
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub description: String,

    #[validate(custom(function = "crate::domains::payments::dtos::validate_payment_amount"))]
    pub amount: Decimal,
    pub due_date: Option<NaiveDate>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdatePaymentRequestDto {
    #[validate(length(max = 255))]
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub description: Option<String>,

    #[validate(custom(function = "crate::domains::payments::dtos::validate_payment_amount"))]
    pub amount: Option<Decimal>,
    #[serde(default)]
    pub due_date: Option<Option<NaiveDate>>,
    pub status: Option<PaymentStatus>,
}

#[derive(Serialize)]
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

pub fn validate_payment_amount(value: &Decimal) -> Result<(), validator::ValidationError> {
    if value <= &Decimal::ZERO
        || value.round_dp(2) != *value
        || value > &Decimal::new(999_999_999_999, 2)
    {
        let mut error = validator::ValidationError::new("invalid_payment_amount");
        error.message = Some(
            "O valor deve ser positivo, ter no máximo duas casas decimais e caber no limite aceito."
                .into(),
        );
        return Err(error);
    }

    Ok(())
}

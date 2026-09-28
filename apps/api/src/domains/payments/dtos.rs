use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::models::PaymentStatus;

#[derive(Serialize)]
pub struct CreatePaymentRequestDto {
    pub project_id: Uuid,
    pub description: String,
    pub amount: Decimal,
    pub due_date: Option<NaiveDate>,
    pub status: PaymentStatus,
}

#[derive(Serialize)]
pub struct UpdatePaymentRequestDto {
    pub description: Option<String>,
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

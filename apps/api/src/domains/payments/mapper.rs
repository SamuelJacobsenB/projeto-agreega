use super::{dtos::PaymentResponseDto, models::Payment};

impl From<Payment> for PaymentResponseDto {
    fn from(payment: Payment) -> Self {
        Self {
            id: payment.id,
            project_id: payment.project_id,
            description: payment.description,
            amount: payment.amount,
            due_date: payment.due_date,
            paid_at: payment.paid_at,
            status: payment.status,
            created_at: payment.created_at,
            updated_at: payment.updated_at,
        }
    }
}

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

/// Representa um pagamento ou compromisso financeiro relacionado a um projeto.
/// Conecta-se a Project.
/// Usado para acompanhar valores, vencimentos e pagamentos do projeto.
pub struct Payment {
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

pub enum PaymentStatus {
    Pending,
    Paid,
    Overdue,
    Cancelled,
}

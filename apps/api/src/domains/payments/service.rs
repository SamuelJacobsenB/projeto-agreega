use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::{
        payments::{
            dtos::{CreatePaymentRequestDto, UpdatePaymentRequestDto},
            models::{Payment, PaymentStatus},
            repository::Repository,
        },
        projects::authorization::authorize_project_access,
        users::models::UserRole,
    },
    response::{AppError, AppResult},
};

pub struct Service;

impl Service {
    pub async fn list_by_project_id(
        pool: &PgPool,
        project_id: Uuid,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Vec<Payment>> {
        authorize_project_access(pool, project_id, user_id, role).await?;
        Repository::find_by_project_id(pool, project_id).await
    }

    pub async fn create(
        pool: &PgPool,
        dto: &CreatePaymentRequestDto,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Payment> {
        authorize_project_access(pool, dto.project_id, user_id, role).await?;
        Repository::create(pool, dto).await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        dto: &UpdatePaymentRequestDto,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<Payment> {
        if dto.description.is_none()
            && dto.amount.is_none()
            && dto.due_date.is_none()
            && dto.status.is_none()
        {
            return Err(AppError::BadRequest(
                "Informe ao menos um campo para atualizar.".to_string(),
            ));
        }

        let payment = Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Pagamento não encontrado.".to_string()))?;
        authorize_project_access(pool, payment.project_id, user_id, role).await?;

        if payment.status == PaymentStatus::Paid
            && (dto.description.is_some()
                || dto.amount.is_some()
                || dto.due_date.is_some()
                || dto
                    .status
                    .is_some_and(|status| status != PaymentStatus::Paid))
        {
            return Err(AppError::Conflict(
                "Pagamentos quitados não podem ser alterados.".to_string(),
            ));
        }

        Repository::update(pool, id, dto).await
    }
}

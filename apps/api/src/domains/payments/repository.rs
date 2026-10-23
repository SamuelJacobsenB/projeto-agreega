use sqlx::PgPool;
use uuid::Uuid;

use crate::response::{AppError, AppResult};

use super::{dtos::CreatePaymentRequestDto, dtos::UpdatePaymentRequestDto, models::Payment};

pub struct Repository;

impl Repository {
    pub async fn find_by_project_id(pool: &PgPool, project_id: Uuid) -> AppResult<Vec<Payment>> {
        sqlx::query_as::<_, Payment>(
            r#"
			SELECT * FROM payments
			WHERE project_id = $1
			ORDER BY created_at DESC, id DESC
			"#,
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar pagamentos.".to_string()))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<Payment>> {
        sqlx::query_as::<_, Payment>("SELECT * FROM payments WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao buscar pagamento.".to_string()))
    }

    pub async fn create(pool: &PgPool, dto: &CreatePaymentRequestDto) -> AppResult<Payment> {
        sqlx::query_as::<_, Payment>(
            r#"
			INSERT INTO payments (
                id, project_id, description, amount, due_date
			)
            VALUES ($1, $2, $3, $4, $5)
			RETURNING *
			"#,
        )
        .bind(Uuid::new_v4())
        .bind(dto.project_id)
        .bind(&dto.description)
        .bind(dto.amount)
        .bind(dto.due_date)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao criar pagamento.".to_string()))
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        dto: &UpdatePaymentRequestDto,
    ) -> AppResult<Payment> {
        sqlx::query_as::<_, Payment>(
            r#"
			UPDATE payments
			SET description = COALESCE($2, description),
				amount = COALESCE($3, amount),
                due_date = CASE WHEN $4 THEN $5 ELSE due_date END,
                status = COALESCE($6, status),
				paid_at = CASE
                    WHEN $6::payment_status IS NULL THEN paid_at
                    WHEN $6 = 'paid'::payment_status THEN COALESCE(paid_at, NOW())
					ELSE NULL
				END,
				updated_at = NOW()
            WHERE id = $1 AND status <> 'paid'
			RETURNING *
			"#,
        )
        .bind(id)
        .bind(&dto.description)
        .bind(dto.amount)
        .bind(dto.due_date.is_some())
        .bind(dto.due_date.flatten())
        .bind(dto.status)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao atualizar pagamento.".to_string()))?
        .ok_or_else(|| AppError::Conflict("Pagamento não encontrado ou já quitado.".to_string()))
    }
}

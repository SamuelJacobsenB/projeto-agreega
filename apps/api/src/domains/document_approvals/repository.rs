use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::audit::repository::Repository as AuditRepository,
    response::{AppError, AppResult},
};

use super::models::{ApprovalStatus, DocumentApproval};

pub struct Repository;

impl Repository {
    pub async fn find_by_document_id(
        pool: &PgPool,
        document_id: Uuid,
    ) -> AppResult<Vec<DocumentApproval>> {
        sqlx::query_as::<_, DocumentApproval>(
            "SELECT * FROM document_approvals WHERE document_id = $1 ORDER BY created_at DESC",
        )
        .bind(document_id)
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar aprovações do documento.".to_string()))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<DocumentApproval>> {
        sqlx::query_as::<_, DocumentApproval>("SELECT * FROM document_approvals WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao buscar aprovação.".to_string()))
    }

    pub async fn create(
        pool: &PgPool,
        document_id: Uuid,
        document_version_id: Uuid,
        requested_by: Uuid,
        comment: Option<&str>,
    ) -> AppResult<DocumentApproval> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar solicitação de aprovação.".to_string())
        })?;

        let document_exists =
            sqlx::query_scalar::<_, Uuid>("SELECT id FROM documents WHERE id = $1 FOR UPDATE")
                .bind(document_id)
                .fetch_optional(&mut *transaction)
                .await
                .map_err(|_| {
                    AppError::Database("Falha ao validar documento da aprovação.".to_string())
                })?;

        if document_exists.is_none() {
            return Err(AppError::NotFound("Documento não encontrado.".to_string()));
        }

        let pending_approval = sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM document_approvals WHERE document_version_id = $1 AND status = 'pending' LIMIT 1",
        )
        .bind(document_version_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao verificar aprovação pendente.".to_string()))?;

        if pending_approval.is_some() {
            return Err(AppError::Conflict(
                "Já existe uma solicitação pendente para esta versão.".to_string(),
            ));
        }

        let approval = sqlx::query_as::<_, DocumentApproval>(
            r#"
				INSERT INTO document_approvals (
					id, document_id, document_version_id, requested_by, comment
				)
				VALUES ($1, $2, $3, $4, $5)
				RETURNING *
			"#,
        )
        .bind(Uuid::new_v4())
        .bind(document_id)
        .bind(document_version_id)
        .bind(requested_by)
        .bind(comment)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao criar solicitação de aprovação.".to_string()))?;

        AuditRepository::create_in_transaction(
            &mut transaction,
            requested_by,
            "document.approval_requested",
            "document",
            document_id,
            None,
        )
        .await?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao confirmar solicitação de aprovação.".to_string())
        })?;

        Ok(approval)
    }

    pub async fn review(
        pool: &PgPool,
        id: Uuid,
        reviewed_by: Uuid,
        status: ApprovalStatus,
        comment: Option<&str>,
    ) -> AppResult<DocumentApproval> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar revisão da aprovação.".to_string())
        })?;

        let approval = sqlx::query_as::<_, DocumentApproval>(
            r#"
				UPDATE document_approvals
				SET status = $2, reviewed_by = $3, comment = COALESCE($4, comment), reviewed_at = NOW()
				WHERE id = $1 AND status = 'pending'
				RETURNING *
			"#,
        )
        .bind(id)
        .bind(status)
        .bind(reviewed_by)
        .bind(comment)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao registrar revisão da aprovação.".to_string()))?;

        let approval = approval
            .ok_or_else(|| AppError::Conflict("Esta aprovação já foi revisada.".to_string()))?;
        let action = match status {
            ApprovalStatus::Approved => "document.approval_approved",
            ApprovalStatus::ChangesRequested => "document.approval_changes_requested",
            ApprovalStatus::Rejected => "document.approval_rejected",
            ApprovalStatus::Pending => unreachable!(),
        };

        AuditRepository::create_in_transaction(
            &mut transaction,
            reviewed_by,
            action,
            "document",
            approval.document_id,
            None,
        )
        .await?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao confirmar revisão da aprovação.".to_string())
        })?;

        Ok(approval)
    }
}

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::audit::repository::Repository as AuditRepository,
    domains::document_versions::models::DocumentVersion,
    response::{AppError, AppResult},
};

pub struct Repository;

impl Repository {
    pub async fn find_by_document_id(
        pool: &PgPool,
        document_id: Uuid,
    ) -> AppResult<Vec<DocumentVersion>> {
        sqlx::query_as::<_, DocumentVersion>(
            "SELECT * FROM document_versions WHERE document_id = $1 ORDER BY version DESC",
        )
        .bind(document_id)
        .fetch_all(pool)
        .await
        .map_err(|_| {
            AppError::Database("Falha ao encontrar versões do documento nos dados.".to_string())
        })
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<DocumentVersion>> {
        sqlx::query_as::<_, DocumentVersion>("SELECT * FROM document_versions WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|_| {
                AppError::Database("Falha ao buscar versão de documento nos dados.".to_string())
            })
    }

    pub async fn create(
        pool: &PgPool,
        document_id: Uuid,
        file_id: Uuid,
        uploaded_by: Uuid,
    ) -> AppResult<DocumentVersion> {
        let mut transaction = pool
            .begin()
            .await
            .map_err(|_| AppError::Database("Falha ao iniciar criação da versão.".to_string()))?;

        let current_version = sqlx::query_scalar::<_, i16>(
            "SELECT current_version FROM documents WHERE id = $1 FOR UPDATE",
        )
        .bind(document_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| {
            AppError::Database("Falha ao verificar versão atual do documento.".to_string())
        })?
        .ok_or_else(|| AppError::NotFound("Documento não encontrado.".to_string()))?;

        let version = current_version.checked_add(1).ok_or_else(|| {
            AppError::BadRequest("O documento atingiu o limite de versões permitido.".to_string())
        })?;

        let document_version = sqlx::query_as::<_, DocumentVersion>(
            r#"
				INSERT INTO document_versions (
					id,
                    document_id,
                    file_id,
                    version,
                    uploaded_by
				)
				VALUES ($1, $2, $3, $4, $5)
				RETURNING *
			"#,
        )
        .bind(Uuid::new_v4())
        .bind(document_id)
        .bind(file_id)
        .bind(version)
        .bind(uploaded_by)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao registrar versão de documento.".to_string()))?;

        sqlx::query("UPDATE documents SET current_version = $2, updated_at = NOW() WHERE id = $1")
            .bind(document_id)
            .bind(version)
            .execute(&mut *transaction)
            .await
            .map_err(|_| {
                AppError::Database("Falha ao atualizar versão atual do documento.".to_string())
            })?;

        AuditRepository::create_in_transaction(
            &mut transaction,
            uploaded_by,
            "document.version_uploaded",
            "document",
            document_id,
            None,
        )
        .await?;

        transaction
            .commit()
            .await
            .map_err(|_| AppError::Database("Falha ao confirmar criação da versão.".to_string()))?;

        Ok(document_version)
    }
}

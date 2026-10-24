use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::audit::repository::Repository as AuditRepository,
    domains::users::{dtos::UpdateUserRequestDto, models::User},
    response::{AppError, AppResult},
};

pub struct Repository;

impl Repository {
    pub async fn find_all(pool: &PgPool) -> AppResult<Vec<User>> {
        sqlx::query_as::<_, User>(
            r#"
                SELECT *
                FROM users
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar usuários nos dados".to_string()))
    }

    pub async fn find_staff(pool: &PgPool) -> AppResult<Vec<User>> {
        sqlx::query_as::<_, User>(
            r#"
                SELECT *
                FROM users
                WHERE role = 'admin' OR role = 'engineer' OR role = 'architect' OR role = 'technical'
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar usuários staff nos dados".to_string()))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<User>> {
        sqlx::query_as::<_, User>(
            r#"
                SELECT *
                FROM users
                WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar usuário nos dados".to_string()))
    }

    pub async fn find_by_phone(pool: &PgPool, phone: &str) -> AppResult<Option<User>> {
        sqlx::query_as::<_, User>(
            r#"
                SELECT *
                FROM users
                WHERE phone = $1
            "#,
        )
        .bind(phone)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar usuário nos dados".to_string()))
    }

    pub async fn find_by_cpf(pool: &PgPool, cpf: &str) -> AppResult<Option<User>> {
        sqlx::query_as::<_, User>(
            r#"
                SELECT *
                FROM users
                WHERE cpf = $1
            "#,
        )
        .bind(cpf)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar usuário nos dados".to_string()))
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        dto: &UpdateUserRequestDto,
        updated_by: Uuid,
    ) -> AppResult<Option<User>> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar atualização do usuário.".to_string())
        })?;

        let user = sqlx::query_as::<_, User>(
            r#"
                UPDATE users
                SET name = $1, phone = $2, cpf = $3
                WHERE id = $4
                RETURNING *
            "#,
        )
        .bind(&dto.name)
        .bind(&dto.phone)
        .bind(&dto.cpf)
        .bind(id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao atualizar usuário nos dados".to_string()))?;

        if user.is_some() {
            AuditRepository::create_in_transaction(
                &mut transaction,
                updated_by,
                "user.updated",
                "user",
                id,
                None,
            )
            .await?;
        }

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao confirmar atualização do usuário.".to_string())
        })?;

        Ok(user)
    }

    pub async fn delete(pool: &PgPool, id: Uuid, deleted_by: Uuid) -> AppResult<()> {
        let mut transaction = pool
            .begin()
            .await
            .map_err(|_| AppError::Database("Falha ao iniciar exclusão do usuário.".to_string()))?;

        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&mut *transaction)
            .await
            .map_err(|_| AppError::Database("Falha ao deletar usuário nos dados".to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Usuário não encontrado.".to_string()));
        }

        AuditRepository::create_in_transaction(
            &mut transaction,
            deleted_by,
            "user.deleted",
            "user",
            id,
            None,
        )
        .await?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao confirmar exclusão do usuário.".to_string())
        })?;

        Ok(())
    }
}

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::{
        auth::models::{PasswordResetToken, RefreshSession},
        users::models::User,
    },
    response::{AppError, AppResult},
};

pub struct Repository;

impl Repository {
    pub async fn find_user_by_email(pool: &PgPool, email: &str) -> AppResult<Option<User>> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE lower(email) = lower($1)")
            .bind(email)
            .fetch_optional(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao buscar usuário nos dados.".to_string()))
    }
}

impl Repository {
    pub async fn create_refresh_session(
        pool: &PgPool,
        user_id: Uuid,
        token_hash: &str,
        expires_at: DateTime<Utc>,
        user_agent: Option<&str>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO refresh_sessions (
                id,
                user_id,
                token_hash,
                expires_at,
                user_agent
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(token_hash)
        .bind(expires_at)
        .bind(user_agent)
        .execute(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao criar sessão nos dados.".to_string()))?;

        Ok(())
    }

    pub async fn rotate_refresh_session(
        pool: &PgPool,
        old_token_hash: &str,
        new_token_hash: &str,
        expires_at: DateTime<Utc>,
        user_agent: Option<&str>,
    ) -> AppResult<User> {
        let mut transaction = pool
            .begin()
            .await
            .map_err(|_| AppError::Database("Falha ao iniciar renovação de sessão.".to_string()))?;

        let session = sqlx::query_as::<_, RefreshSession>(
            r#"
            SELECT
                id,
                user_id,
                token_hash,
                expires_at,
                revoked_at,
                user_agent,
                ip_address::text AS ip_address,
                created_at
            FROM refresh_sessions
            WHERE token_hash = $1
              AND revoked_at IS NULL
              AND expires_at > NOW()
            FOR UPDATE
            "#,
        )
        .bind(old_token_hash)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar sessão nos dados.".to_string()))?
        .ok_or_else(|| AppError::Unauthorized("Sessão inválida ou expirada.".to_string()))?;

        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(session.user_id)
            .fetch_optional(&mut *transaction)
            .await
            .map_err(|_| AppError::Database("Falha ao buscar usuário nos dados.".to_string()))?
            .ok_or_else(|| AppError::Unauthorized("Usuário não encontrado.".to_string()))?;

        sqlx::query(
            "UPDATE refresh_sessions
             SET revoked_at = NOW()
             WHERE id = $1",
        )
        .bind(session.id)
        .execute(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao revogar sessão nos dados.".to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO refresh_sessions (
                id,
                user_id,
                token_hash,
                expires_at,
                user_agent
            )
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user.id)
        .bind(new_token_hash)
        .bind(expires_at)
        .bind(user_agent)
        .execute(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao criar nova sessão nos dados.".to_string()))?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao concluir renovação de sessão.".to_string())
        })?;

        Ok(user)
    }

    pub async fn revoke_refresh_session(pool: &PgPool, token_hash: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE refresh_sessions
            SET revoked_at = NOW()
            WHERE token_hash = $1
              AND revoked_at IS NULL
            "#,
        )
        .bind(token_hash)
        .execute(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao revogar sessão nos dados.".to_string()))?;

        Ok(())
    }
}

impl Repository {
    pub async fn create_password_reset_token(
        pool: &PgPool,
        user_id: Uuid,
        token_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> AppResult<()> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar recuperação de senha.".to_string())
        })?;

        sqlx::query(
            r#"
            UPDATE password_reset_tokens
            SET used_at = NOW()
            WHERE user_id = $1
              AND used_at IS NULL
            "#,
        )
        .bind(user_id)
        .execute(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao invalidar tokens anteriores.".to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO password_reset_tokens (
                id,
                user_id,
                token_hash,
                expires_at
            )
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(token_hash)
        .bind(expires_at)
        .execute(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao criar token de recuperação.".to_string()))?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao concluir recuperação de senha.".to_string())
        })?;

        Ok(())
    }

    pub async fn reset_password(
        pool: &PgPool,
        token_hash: &str,
        password_hash: &str,
    ) -> AppResult<()> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar redefinição de senha.".to_string())
        })?;

        let token = sqlx::query_as::<_, PasswordResetToken>(
            r#"
            SELECT *
            FROM password_reset_tokens
            WHERE token_hash = $1
              AND used_at IS NULL
              AND expires_at > NOW()
            FOR UPDATE
            "#,
        )
        .bind(token_hash)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar token nos dados.".to_string()))?
        .ok_or_else(|| AppError::Unauthorized("Token inválido ou expirado.".to_string()))?;

        sqlx::query(
            r#"
            UPDATE users
            SET password_hash = $1,
                updated_at = NOW()
            WHERE id = $2
            "#,
        )
        .bind(password_hash)
        .bind(token.user_id)
        .execute(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao redefinir senha nos dados.".to_string()))?;

        sqlx::query(
            r#"
            UPDATE password_reset_tokens
            SET used_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(token.id)
        .execute(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao consumir token nos dados.".to_string()))?;

        sqlx::query(
            r#"
            UPDATE refresh_sessions
            SET revoked_at = NOW()
            WHERE user_id = $1
              AND revoked_at IS NULL
            "#,
        )
        .bind(token.user_id)
        .execute(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao encerrar sessões do usuário.".to_string()))?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao concluir redefinição de senha.".to_string())
        })?;

        Ok(())
    }
}

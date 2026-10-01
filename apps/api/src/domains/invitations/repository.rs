use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::{
        invitations::{dtos::CreateInvitationRequestDto, models::Invitation},
        users::models::User,
    },
    response::{AppError, AppResult},
};

pub struct Repository;

impl Repository {
    pub async fn email_exists(pool: &PgPool, email: &str) -> AppResult<bool> {
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(
                SELECT 1
                FROM users
                WHERE lower(email) = lower($1)
            )",
        )
        .bind(email)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao verificar e-mail nos dados.".to_string()))
    }
}

impl Repository {
    pub async fn pending_invitation_exists(pool: &PgPool, email: &str) -> AppResult<bool> {
        sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM invitations
                WHERE lower(email) = lower($1)
                  AND accepted_at IS NULL
                  AND expires_at > NOW()
            )
            "#,
        )
        .bind(email)
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao verificar convite nos dados.".to_string()))
    }

    pub async fn find_all_invitations(pool: &PgPool) -> AppResult<Vec<Invitation>> {
        sqlx::query_as::<_, Invitation>(
            r#"
            SELECT *
            FROM invitations
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar convites nos dados.".to_string()))
    }

    pub async fn find_invitation_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<Invitation>> {
        sqlx::query_as::<_, Invitation>("SELECT * FROM invitations WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao buscar convite nos dados.".to_string()))
    }

    pub async fn create_invitation(
        pool: &PgPool,
        dto: &CreateInvitationRequestDto,
        invited_by: Uuid,
        token_hash: &str,
        expires_at: DateTime<Utc>,
    ) -> AppResult<Invitation> {
        let mut transaction = pool
            .begin()
            .await
            .map_err(|_| AppError::Database("Falha ao iniciar criação de convite.".to_string()))?;

        sqlx::query(
            r#"
            DELETE FROM invitations
            WHERE lower(email) = lower($1)
              AND accepted_at IS NULL
              AND expires_at <= NOW()
            "#,
        )
        .bind(dto.email.trim())
        .execute(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao remover convite expirado.".to_string()))?;

        let invitation = sqlx::query_as::<_, Invitation>(
            r#"
            INSERT INTO invitations (
                id,
                email,
                invited_by,
                client_id,
                role,
                token_hash,
                expires_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(dto.email.trim().to_lowercase())
        .bind(invited_by)
        .bind(dto.client_id)
        .bind(dto.role)
        .bind(token_hash)
        .bind(expires_at)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao criar convite no banco de dados.".to_string()))?;

        transaction
            .commit()
            .await
            .map_err(|_| AppError::Database("Falha ao concluir criação de convite.".to_string()))?;

        Ok(invitation)
    }

    pub async fn accept_invitation(
        pool: &PgPool,
        token_hash: &str,
        name: &str,
        phone: &str,
        password_hash: &str,
    ) -> AppResult<User> {
        let mut transaction = pool
            .begin()
            .await
            .map_err(|_| AppError::Database("Falha ao iniciar criação da conta.".to_string()))?;

        let invitation = sqlx::query_as::<_, Invitation>(
            r#"
            SELECT *
            FROM invitations
            WHERE token_hash = $1
              AND accepted_at IS NULL
              AND expires_at > NOW()
            FOR UPDATE
            "#,
        )
        .bind(token_hash)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar convite nos dados.".to_string()))?
        .ok_or_else(|| AppError::Unauthorized("Convite inválido ou expirado.".to_string()))?;

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                id,
                name,
                email,
                phone,
                password_hash,
                role,
                client_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(name.trim())
        .bind(invitation.email)
        .bind(phone)
        .bind(password_hash)
        .bind(invitation.role)
        .bind(invitation.client_id)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|error| {
            if error
                .as_database_error()
                .and_then(|database_error| database_error.code())
                .as_deref()
                == Some("23505")
            {
                AppError::Conflict("E-mail ou telefone já cadastrado.".to_string())
            } else {
                AppError::Database("Falha ao criar usuário nos dados.".to_string())
            }
        })?;

        sqlx::query(
            "UPDATE invitations
             SET accepted_at = NOW()
             WHERE id = $1",
        )
        .bind(invitation.id)
        .execute(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao consumir convite nos dados.".to_string()))?;

        transaction
            .commit()
            .await
            .map_err(|_| AppError::Database("Falha ao concluir criação da conta.".to_string()))?;

        Ok(user)
    }

    pub async fn delete_invitation(pool: &PgPool, id: Uuid) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM invitations WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|_| AppError::Database("Falha ao deletar convite dos dados.".to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Convite não encontrado.".to_string()));
        }

        Ok(())
    }
}

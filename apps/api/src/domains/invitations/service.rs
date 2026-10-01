use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use chrono::{DateTime, Duration, Utc};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::{
        invitations::{
            dtos::{AcceptInvitationRequestDto, CreateInvitationRequestDto},
            models::Invitation,
            repository::Repository,
        },
        users::models::User,
    },
    response::{AppError, AppResult},
};

const INVITATION_DAYS: i64 = 7;

pub struct Service;

impl Service {
    pub async fn list_invitations(pool: &PgPool) -> AppResult<Vec<Invitation>> {
        Repository::find_all_invitations(pool).await
    }

    pub async fn create_invitation(
        pool: &PgPool,
        dto: &CreateInvitationRequestDto,
        invited_by: Uuid,
    ) -> AppResult<(Invitation, String)> {
        Self::ensure_email_available(pool, &dto.email).await?;

        let token = new_token();
        let token_hash = hash_token(&token);
        let expires_at = expires_in_days(INVITATION_DAYS);

        let invitation =
            Repository::create_invitation(pool, dto, invited_by, &token_hash, expires_at).await?;

        Ok((invitation, token))
    }

    pub async fn accept_invitation(
        pool: &PgPool,
        dto: &AcceptInvitationRequestDto,
    ) -> AppResult<User> {
        let password_hash = hash_password(&dto.password)?;
        let token_hash = hash_token(&dto.token);

        Repository::accept_invitation(pool, &token_hash, &dto.name, &dto.phone, &password_hash)
            .await
    }

    pub async fn delete_invitation(pool: &PgPool, id: Uuid) -> AppResult<()> {
        Repository::delete_invitation(pool, id).await
    }

    async fn ensure_email_available(pool: &PgPool, email: &str) -> AppResult<()> {
        if Repository::email_exists(pool, email).await? {
            return Err(AppError::Conflict(
                "Este e-mail já possui uma conta.".to_string(),
            ));
        }

        if Repository::pending_invitation_exists(pool, email).await? {
            return Err(AppError::Conflict(
                "Já existe um convite pendente para este e-mail.".to_string(),
            ));
        }

        Ok(())
    }
}

fn new_token() -> String {
    Uuid::new_v4().simple().to_string()
}

fn hash_token(token: &str) -> String {
    hex::encode(Sha256::digest(token.as_bytes()))
}

fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| AppError::Internal("Falha ao proteger senha.".to_string()))
}

fn expires_in_days(days: i64) -> DateTime<Utc> {
    Utc::now() + Duration::days(days)
}

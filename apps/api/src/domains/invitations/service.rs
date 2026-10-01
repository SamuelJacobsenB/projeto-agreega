use chrono::{Duration, Utc};
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
    infrastructure::email::EmailService,
    response::{AppError, AppResult},
    security::{password::PasswordService, token::TokenService},
};

const INVITATION_DAYS: i64 = 7;

pub struct Service;

impl Service {
    pub async fn list_invitations(pool: &PgPool) -> AppResult<Vec<Invitation>> {
        Repository::find_all_invitations(pool).await
    }

    pub async fn create_invitation(
        pool: &PgPool,
        email_service: &EmailService,
        dto: &CreateInvitationRequestDto,
        invited_by: Uuid,
    ) -> AppResult<Invitation> {
        Self::ensure_email_available(pool, &dto.email).await?;

        let token = TokenService::new_token()?;
        let token_hash = TokenService::hash_token(&token);
        let expires_at = Utc::now() + Duration::days(INVITATION_DAYS);

        let invitation =
            Repository::create_invitation(pool, dto, invited_by, &token_hash, expires_at).await?;

        email_service
            .send_invitation_email(&dto.email, &token)
            .await?;

        Ok(invitation)
    }

    pub async fn accept_invitation(
        pool: &PgPool,
        dto: &AcceptInvitationRequestDto,
    ) -> AppResult<User> {
        let password_hash = PasswordService::hash_password(&dto.password)?;
        let token_hash = TokenService::hash_token(&dto.token);

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

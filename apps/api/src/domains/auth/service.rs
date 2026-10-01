use chrono::{Duration, Utc};
use sqlx::PgPool;

use crate::{
    domains::{
        auth::{dtos::AuthTokens, repository::Repository},
        users::models::User,
    },
    response::{AppError, AppResult},
    security::{jwt::JwtService, password::PasswordService, token::TokenService},
};

const REFRESH_TOKEN_DAYS: i64 = 7;
const PASSWORD_RESET_MINUTES: i64 = 15;

pub struct Service;

impl Service {
    pub async fn login(
        pool: &PgPool,
        email: &str,
        password: &str,
        jwt_secret: &str,
        user_agent: Option<&str>,
    ) -> AppResult<AuthTokens> {
        let user = Repository::find_user_by_email(pool, email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("E-mail ou senha inválidos.".to_string()))?;

        PasswordService::verify_password(password, &user.password_hash)?;

        Self::create_tokens(pool, &user, jwt_secret, user_agent).await
    }

    pub async fn refresh(
        pool: &PgPool,
        refresh_token: &str,
        jwt_secret: &str,
        user_agent: Option<&str>,
    ) -> AppResult<AuthTokens> {
        let new_refresh_token = TokenService::new_token()?;

        let user = Repository::rotate_refresh_session(
            pool,
            &TokenService::hash_token(refresh_token),
            &TokenService::hash_token(&new_refresh_token),
            Utc::now() + Duration::days(REFRESH_TOKEN_DAYS),
            user_agent,
        )
        .await?;

        let (access_token, expires_at) = JwtService::create_access_token(&user, jwt_secret)?;

        Ok(AuthTokens {
            access_token,
            refresh_token: new_refresh_token,
            expires_at,
        })
    }

    pub async fn logout(pool: &PgPool, refresh_token: &str) -> AppResult<()> {
        Repository::revoke_refresh_session(pool, &TokenService::hash_token(refresh_token)).await
    }

    pub async fn request_password_reset(pool: &PgPool, email: &str) -> AppResult<Option<String>> {
        let Some(user) = Repository::find_user_by_email(pool, email).await? else {
            return Ok(None);
        };

        let token = TokenService::new_token()?;

        Repository::create_password_reset_token(
            pool,
            user.id,
            &TokenService::hash_token(&token),
            Utc::now() + Duration::minutes(PASSWORD_RESET_MINUTES),
        )
        .await?;

        Ok(Some(token))
    }

    pub async fn reset_password(pool: &PgPool, token: &str, password: &str) -> AppResult<()> {
        let password_hash = PasswordService::hash_password(password)?;

        Repository::reset_password(pool, &TokenService::hash_token(token), &password_hash).await
    }

    async fn create_tokens(
        pool: &PgPool,
        user: &User,
        jwt_secret: &str,
        user_agent: Option<&str>,
    ) -> AppResult<AuthTokens> {
        let refresh_token = TokenService::new_token()?;

        Repository::create_refresh_session(
            pool,
            user.id,
            &TokenService::hash_token(&refresh_token),
            Utc::now() + Duration::days(REFRESH_TOKEN_DAYS),
            user_agent,
        )
        .await?;

        let (access_token, expires_at) = JwtService::create_access_token(user, jwt_secret)?;

        Ok(AuthTokens {
            access_token,
            refresh_token,
            expires_at,
        })
    }
}

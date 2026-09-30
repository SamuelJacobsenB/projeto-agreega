use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domains::users::models::UserRole;

#[derive(Deserialize, Validate)]
pub struct LoginRequestDto {
    #[validate(email(message = "E-mail inválido."))]
    pub email: String,

    #[validate(custom(function = "crate::validation::validate_password"))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RefreshTokenRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub refresh_token: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateInvitationRequestDto {
    #[validate(email(message = "E-mail inválido."))]
    pub email: String,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub client_id: Option<Uuid>,
    pub role: UserRole,
}

#[derive(Deserialize, Validate)]
pub struct AcceptInvitationRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub token: String,

    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub name: String,

    #[validate(custom(function = "crate::validation::validate_password"))]
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct RequestPasswordResetRequestDto {
    #[validate(email(message = "E-mail inválido."))]
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct ResetPasswordRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub token: String,

    #[validate(custom(function = "crate::validation::validate_password"))]
    pub password: String,
}

#[derive(Deserialize)]
pub struct AuthTokenResponseDto {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct InvitationResponseDto {
    pub id: Uuid,
    pub email: String,
    pub invited_by: Uuid,
    pub client_id: Option<Uuid>,
    pub role: UserRole,
    pub expires_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

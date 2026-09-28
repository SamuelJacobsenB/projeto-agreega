use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domains::users::models::UserRole;

#[derive(Serialize)]
pub struct LoginRequestDto {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RefreshTokenRequestDto {
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct CreateInvitationRequestDto {
    pub email: String,
    pub client_id: Option<Uuid>,
    pub role: UserRole,
}

#[derive(Serialize)]
pub struct AcceptInvitationRequestDto {
    pub token: String,
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RequestPasswordResetRequestDto {
    pub email: String,
}

#[derive(Serialize)]
pub struct ResetPasswordRequestDto {
    pub token: String,
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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginRequestDto {
    #[validate(email(message = "E-mail inválido."))]
    #[validate(length(max = 255, message = "E-mail deve ter no máximo 255 caracteres."))]
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
pub struct RequestPasswordResetRequestDto {
    #[validate(email(message = "E-mail inválido."))]
    #[validate(length(max = 255, message = "E-mail deve ter no máximo 255 caracteres."))]
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct ResetPasswordRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub token: String,

    #[validate(custom(function = "crate::validation::validate_password"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthTokenResponseDto {
    pub access_token: String,
    pub expires_at: DateTime<Utc>,
}

pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

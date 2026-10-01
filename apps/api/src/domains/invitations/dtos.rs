use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::domains::users::models::UserRole;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateInvitationRequestDto {
    #[validate(email(message = "E-mail inválido."))]
    #[validate(length(max = 255, message = "E-mail deve ter no máximo 255 caracteres."))]
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
    #[validate(length(
        min = 2,
        max = 100,
        message = "Nome deve ter entre 2 e 100 caracteres."
    ))]
    pub name: String,

    #[validate(custom(function = "crate::validation::validate_brazilian_phone"))]
    pub phone: String,

    #[validate(custom(function = "crate::validation::validate_cpf"))]
    pub cpf: Option<String>,

    #[validate(custom(function = "crate::validation::validate_password"))]
    pub password: String,
}

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct CreatedInvitationResponseDto {
    pub invitation: InvitationResponseDto,
    pub token: String,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::models::UserRole;

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateUserRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub name: Option<String>,

    #[validate(custom(function = "crate::validation::validate_brazilian_phone"))]
    pub phone: Option<String>,

    #[validate(custom(function = "crate::validation::validate_cpf"))]
    pub cpf: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub cpf: Option<String>,
    pub role: UserRole,
    pub client_id: Option<Uuid>,
    pub avatar_file_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

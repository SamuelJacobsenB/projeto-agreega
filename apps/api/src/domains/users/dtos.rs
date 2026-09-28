use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::models::UserRole;

#[derive(Serialize)]
pub struct CreateUserRequestDto {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub client_id: Option<Uuid>,
    pub avatar_file_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct UpdateUserRequestDto {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<UserRole>,
    pub client_id: Option<Uuid>,
    pub avatar_file_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct UserResponseDto {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub client_id: Option<Uuid>,
    pub avatar_file_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

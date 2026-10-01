use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Representa uma pessoa que possui acesso ao sistema.
/// Conecta-se a Client (opcional), Invitation, RefreshSession, PasswordResetToken,
/// arquivos enviados e às ações realizadas pelo usuário.
/// Usado em autenticação, autorização e identificação de quem executou cada ação.
#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,

    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub cpf: Option<String>,
    pub password_hash: String,

    pub role: UserRole,

    pub client_id: Option<Uuid>,
    pub avatar_file_id: Option<Uuid>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Admin,
    Engineer,
    Architect,
    Technical,
    Client,
}

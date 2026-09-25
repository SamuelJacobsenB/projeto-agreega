use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa uma pessoa que possui acesso ao sistema.
/// Conecta-se a Client (opcional), Invitation, RefreshSession, PasswordResetToken,
/// arquivos enviados e às ações realizadas pelo usuário.
/// Usado em autenticação, autorização e identificação de quem executou cada ação.
pub struct User {
    pub id: Uuid,

    pub name: String,
    pub email: String,
    pub password_hash: String,

    pub role: UserRole,
    pub client_id: Option<Uuid>,

    pub avatar_file_id: Option<Uuid>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum UserRole {
    Admin,
    Engineer,
    Architect,
    Technical,
    Client,
}

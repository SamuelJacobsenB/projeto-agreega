use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Representa uma sessão de autenticação persistente de um usuário.
/// Conecta-se a User.
/// Usado para renovar o acesso sem exigir login novamente e permitir revogação de sessões.
#[derive(sqlx::FromRow)]
pub struct RefreshSession {
    pub id: Uuid,

    pub user_id: Uuid,
    pub token_hash: String,

    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,

    pub user_agent: Option<String>,
    pub ip_address: Option<String>,

    pub created_at: DateTime<Utc>,
}

/// Representa um token temporário para recuperação de senha.
/// Conecta-se a User.
/// Usado no fluxo de "Esqueci minha senha".
#[derive(sqlx::FromRow)]
pub struct PasswordResetToken {
    pub id: Uuid,

    pub user_id: Uuid,
    pub token_hash: String,

    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
}

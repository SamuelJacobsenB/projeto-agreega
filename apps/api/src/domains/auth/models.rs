use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domains::users::models::UserRole;

/// Representa um convite para criação de uma conta de usuário.
/// Conecta-se a User, Client e ao usuário que realizou o convite.
/// Usado quando a Agreega cria uma nova conta sem permitir cadastro público.
pub struct Invitation {
    pub id: Uuid,

    pub email: String,
    pub invited_by: Uuid,

    pub client_id: Option<Uuid>,
    pub role: UserRole,

    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
}

/// Representa uma sessão de autenticação persistente de um usuário.
/// Conecta-se a User.
/// Usado para renovar o acesso sem exigir login novamente e permitir revogação de sessões.
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
pub struct PasswordResetToken {
    pub id: Uuid,

    pub user_id: Uuid,
    pub token_hash: String,

    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
}

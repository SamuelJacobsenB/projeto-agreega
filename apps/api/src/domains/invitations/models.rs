use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domains::users::models::UserRole;

/// Representa um convite para criação de uma conta de usuário.
/// Conecta-se a User, Client e ao usuário que realizou o convite.
/// Usado quando a Agreega cria uma nova conta sem permitir cadastro público.
#[derive(sqlx::FromRow)]
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

use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::role::Role;

pub struct User {
    pub id: Uuid,

    pub name: String,
    pub email: String,
    pub phone: String,
    pub password_hash: String,
    pub role: Role,
    pub avatar_url: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

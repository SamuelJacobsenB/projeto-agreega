use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Client {
    pub id: Uuid,
    pub user_id: Uuid,

    pub company_name: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct File {
    pub id: Uuid,

    pub storage_key: String,
    pub original_name: String,

    pub mime_type: String,
    pub size_bytes: i64,

    pub checksum: Option<String>,

    pub uploaded_by: Uuid,

    pub created_at: DateTime<Utc>,
}

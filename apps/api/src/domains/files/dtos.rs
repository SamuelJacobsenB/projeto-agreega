use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreateFileRequestDto {
    pub storage_key: String,
    pub original_name: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub checksum: Option<String>,
    pub uploaded_by: Uuid,
}

#[derive(Deserialize)]
pub struct FileResponseDto {
    pub id: Uuid,
    pub storage_key: String,
    pub original_name: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub checksum: Option<String>,
    pub uploaded_by: Uuid,
    pub created_at: DateTime<Utc>,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateFileRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub storage_key: String,

    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub original_name: String,

    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub mime_type: String,

    #[validate(custom(function = "crate::validation::validate_positive_i64"))]
    pub size_bytes: i64,
    pub checksum: Option<String>,

    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
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

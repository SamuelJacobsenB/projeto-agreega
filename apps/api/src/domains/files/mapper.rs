use super::{dtos::FileResponseDto, models::File};

impl From<File> for FileResponseDto {
    fn from(file: File) -> Self {
        Self {
            id: file.id,
            storage_key: file.storage_key,
            original_name: file.original_name,
            mime_type: file.mime_type,
            size_bytes: file.size_bytes,
            checksum: file.checksum,
            uploaded_by: file.uploaded_by,
            created_at: file.created_at,
        }
    }
}

use super::{dtos::DocumentVersionResponseDto, models::DocumentVersion};

impl From<DocumentVersion> for DocumentVersionResponseDto {
    fn from(document_version: DocumentVersion) -> Self {
        Self {
            id: document_version.id,
            document_id: document_version.document_id,
            file_id: document_version.file_id,
            version: document_version.version,
            uploaded_by: document_version.uploaded_by,
            created_at: document_version.created_at,
        }
    }
}

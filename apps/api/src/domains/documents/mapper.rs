use super::{dtos::DocumentResponseDto, models::Document};

impl From<Document> for DocumentResponseDto {
    fn from(document: Document) -> Self {
        Self {
            id: document.id,
            project_id: document.project_id,
            stage_id: document.stage_id,
            task_id: document.task_id,
            name: document.name,
            document_type: document.document_type,
            status: document.status,
            current_version: document.current_version,
            created_by: document.created_by,
            created_at: document.created_at,
            updated_at: document.updated_at,
        }
    }
}

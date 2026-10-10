use uuid::Uuid;

use super::{dtos::PhotoResponseDto, models::Photo};

impl PhotoResponseDto {
    pub fn from_model(photo: Photo) -> Self {
        Self {
            id: photo.id,
            project_id: photo.project_id,
            stage_id: photo.stage_id,
            task_id: photo.task_id,
            file_id: photo.file_id,
            description: photo.description,
            url: Self::url(photo.id),
            uploaded_by: photo.uploaded_by,
            created_at: photo.created_at,
        }
    }

    fn url(id: Uuid) -> String {
        format!("/photos/{id}/file")
    }
}

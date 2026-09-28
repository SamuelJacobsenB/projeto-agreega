use super::{dtos::PhotoResponseDto, models::Photo};

impl From<Photo> for PhotoResponseDto {
    fn from(photo: Photo) -> Self {
        Self {
            id: photo.id,
            project_id: photo.project_id,
            stage_id: photo.stage_id,
            task_id: photo.task_id,
            file_id: photo.file_id,
            description: photo.description,
            uploaded_by: photo.uploaded_by,
            created_at: photo.created_at,
        }
    }
}

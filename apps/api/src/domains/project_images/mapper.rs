use super::{dtos::ProjectImageResponseDto, models::ProjectImage};

impl From<ProjectImage> for ProjectImageResponseDto {
    fn from(image: ProjectImage) -> Self {
        Self {
            id: image.id,
            project_id: image.project_id,
            file_id: image.file_id,
            order: image.order,
            created_at: image.created_at,
            updated_at: image.updated_at,
        }
    }
}

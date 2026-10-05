use uuid::Uuid;

use super::{dtos::ProjectImageResponseDto, models::ProjectImage};

impl ProjectImageResponseDto {
    pub fn from_model(image: ProjectImage) -> Self {
        Self {
            id: image.id,
            project_id: image.project_id,
            file_id: image.file_id,
            url: Self::url(image.id),
            order: image.order,
            created_at: image.created_at,
            updated_at: image.updated_at,
        }
    }

    pub fn url(id: Uuid) -> String {
        format!("/project-image/{id}/file")
    }
}

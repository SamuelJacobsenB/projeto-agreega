use super::{dtos::ProjectResponseDto, models::Project};

impl From<Project> for ProjectResponseDto {
    fn from(project: Project) -> Self {
        Self {
            id: project.id,
            client_id: project.client_id,
            name: project.name,
            description: project.description,
            project_type: project.project_type,
            status: project.status,
            address: project.address,
            city: project.city,
            state: project.state,
            start_date: project.start_date,
            estimated_end_date: project.estimated_end_date,
            completed_at: project.completed_at,
            cover_file_id: project.cover_file_id,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
}

use super::{dtos::StageResponseDto, models::Stage};

impl From<Stage> for StageResponseDto {
    fn from(stage: Stage) -> Self {
        Self {
            id: stage.id,
            project_id: stage.project_id,
            name: stage.name,
            description: stage.description,
            weight: stage.weight,
            order: stage.order,
            status: stage.status,
            start_date: stage.start_date,
            end_date: stage.end_date,
            created_at: stage.created_at,
            updated_at: stage.updated_at,
        }
    }
}

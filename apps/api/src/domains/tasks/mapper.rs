use super::{dtos::TaskResponseDto, models::Task};

impl From<Task> for TaskResponseDto {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            stage_id: task.stage_id,
            title: task.title,
            description: task.description,
            weight: task.weight,
            progress: task.progress,
            order: task.order,
            status: task.status,
            assigned_to: task.assigned_to,
            due_date: task.due_date,
            completed_at: task.completed_at,
            created_at: task.created_at,
            updated_at: task.updated_at,
        }
    }
}

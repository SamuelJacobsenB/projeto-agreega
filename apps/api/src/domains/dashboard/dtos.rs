use chrono::NaiveDate;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize)]
pub struct DashboardResponseDto {
    pub statistics: DashboardStatisticsDto,
    pub projects: Vec<DashboardProjectDto>,
    pub tasks: Vec<DashboardTaskDto>,
}

#[derive(Serialize, FromRow)]
pub struct DashboardStatisticsDto {
    pub active_projects: i64,
    pub pending_tasks: i64,
    pub overdue_tasks: i64,
    pub completed_projects: i64,
}

#[derive(Serialize, FromRow)]
pub struct DashboardProjectDto {
    pub id: Uuid,
    pub name: String,
    pub status: String,
    pub city: String,
    pub state: String,
    pub estimated_end_date: Option<NaiveDate>,
    pub progress: Option<i32>,
    pub cover_image_url: Option<String>,
}

#[derive(Serialize, FromRow)]
pub struct DashboardTaskDto {
    pub id: Uuid,
    pub title: String,
    pub status: String,
    pub due_date: Option<NaiveDate>,
    pub is_overdue: bool,
    pub stage_name: String,
    pub project_name: String,
}

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::{
        dashboard::{dtos::DashboardResponseDto, repository::Repository},
        users::models::UserRole,
    },
    response::AppResult,
};

pub struct Service;

impl Service {
    pub async fn get_dashboard(
        pool: &PgPool,
        user_id: Uuid,
        role: UserRole,
    ) -> AppResult<DashboardResponseDto> {
        Repository::get_dashboard(pool, user_id, matches!(role, UserRole::Client)).await
    }
}

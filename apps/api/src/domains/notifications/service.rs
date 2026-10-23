use sqlx::PgPool;
use uuid::Uuid;

use crate::response::AppResult;

use super::{
    dtos::{CreateNotificationRequestDto, ListNotificationsQueryDto, UpdateNotificationRequestDto},
    models::Notification,
    repository::Repository,
};

pub struct Service;

impl Service {
    pub async fn list_for_user(
        pool: &PgPool,
        user_id: Uuid,
        query: &ListNotificationsQueryDto,
    ) -> AppResult<Vec<Notification>> {
        let limit = query.limit.unwrap_or(20).clamp(1, 100);
        let offset = query.offset.unwrap_or(0).max(0);

        Repository::find_by_user_id(pool, user_id, limit, offset).await
    }

    pub async fn create(
        pool: &PgPool,
        dto: &CreateNotificationRequestDto,
    ) -> AppResult<Notification> {
        Repository::create(pool, dto).await
    }

    pub async fn update_read_state(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        dto: &UpdateNotificationRequestDto,
    ) -> AppResult<Notification> {
        Repository::update_read_state(pool, id, user_id, dto.read).await
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> AppResult<()> {
        Repository::delete(pool, id, user_id).await
    }
}

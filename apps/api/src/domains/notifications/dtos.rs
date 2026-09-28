use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::models::NotificationType;

#[derive(Serialize)]
pub struct CreateNotificationRequestDto {
    pub user_id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
}

#[derive(Serialize)]
pub struct UpdateNotificationRequestDto {
    pub read: bool,
}

#[derive(Deserialize)]
pub struct NotificationResponseDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

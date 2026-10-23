use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::models::NotificationType;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateNotificationRequestDto {
    #[validate(custom(function = "crate::validation::validate_not_nil_uuid"))]
    pub user_id: Uuid,

    #[validate(length(max = 100))]
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub title: String,

    #[validate(length(max = 2000))]
    #[validate(custom(function = "crate::validation::validate_not_blank"))]
    pub message: String,
    pub notification_type: NotificationType,
}

#[derive(Deserialize)]
pub struct ListNotificationsQueryDto {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UpdateNotificationRequestDto {
    pub read: bool,
}

#[derive(Serialize)]
pub struct NotificationResponseDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

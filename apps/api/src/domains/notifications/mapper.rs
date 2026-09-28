use super::{dtos::NotificationResponseDto, models::Notification};

impl From<Notification> for NotificationResponseDto {
    fn from(notification: Notification) -> Self {
        Self {
            id: notification.id,
            user_id: notification.user_id,
            title: notification.title,
            message: notification.message,
            notification_type: notification.notification_type,
            read_at: notification.read_at,
            created_at: notification.created_at,
        }
    }
}

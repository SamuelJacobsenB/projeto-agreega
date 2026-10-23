use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    domains::notifications::{
        dtos::{ListNotificationsQueryDto, NotificationResponseDto, UpdateNotificationRequestDto},
        service::Service,
    },
    response::{ApiResponse, ApiResult, ValidatedJson},
    security::auth_user::AuthUser,
};

pub struct Handler;

impl Handler {
    pub async fn list_for_user(
        State(state): State<AppState>,
        AuthUser { id: user_id, .. }: AuthUser,
        Query(query): Query<ListNotificationsQueryDto>,
    ) -> ApiResult<Vec<NotificationResponseDto>> {
        let notifications = Service::list_for_user(&state.pool, user_id, &query).await?;

        let notification_dtos = notifications
            .into_iter()
            .map(NotificationResponseDto::from)
            .collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Notificações encontradas.", Some(notification_dtos)),
        ))
    }

    pub async fn update_read_state(
        State(state): State<AppState>,
        AuthUser { id: user_id, .. }: AuthUser,
        Path(id): Path<Uuid>,
        ValidatedJson(dto): ValidatedJson<UpdateNotificationRequestDto>,
    ) -> ApiResult<NotificationResponseDto> {
        let notification = Service::update_read_state(&state.pool, id, user_id, &dto).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Notificação atualizada.",
                Some(NotificationResponseDto::from(notification)),
            ),
        ))
    }

    pub async fn delete(
        State(state): State<AppState>,
        AuthUser { id: user_id, .. }: AuthUser,
        Path(id): Path<Uuid>,
    ) -> ApiResult<()> {
        Service::delete(&state.pool, id, user_id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Notificação excluída.", None),
        ))
    }
}

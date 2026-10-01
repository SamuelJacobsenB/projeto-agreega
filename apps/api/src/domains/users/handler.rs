use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    domains::users::{
        dtos::{UpdateUserRequestDto, UserResponseDto},
        service::Service,
    },
    response::{ApiResponse, ApiResult, ValidatedJson},
    security::auth_user::AuthUser,
};

pub struct Handler;

impl Handler {
    pub async fn get_all_users(State(state): State<AppState>) -> ApiResult<Vec<UserResponseDto>> {
        let users = Service::get_all_users(&state.pool).await?;

        let user_dtos: Vec<UserResponseDto> =
            users.into_iter().map(UserResponseDto::from).collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Usuários buscados com sucesso.", Some(user_dtos)),
        ))
    }

    pub async fn get_staff_users(State(state): State<AppState>) -> ApiResult<Vec<UserResponseDto>> {
        let users = Service::get_staff_users(&state.pool).await?;

        let user_dtos: Vec<UserResponseDto> =
            users.into_iter().map(UserResponseDto::from).collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Usuários buscados com sucesso.", Some(user_dtos)),
        ))
    }

    pub async fn get_user_by_id(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> ApiResult<UserResponseDto> {
        let user = Service::get_user_by_id(&state.pool, id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Usuário encontrado com sucesso.",
                Some(UserResponseDto::from(user)),
            ),
        ))
    }

    pub async fn get_my_user(
        State(state): State<AppState>,
        AuthUser { id: user_id, .. }: AuthUser,
    ) -> ApiResult<UserResponseDto> {
        let user = Service::get_user_by_id(&state.pool, user_id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Usuário encontrado com sucesso.",
                Some(UserResponseDto::from(user)),
            ),
        ))
    }

    pub async fn update_user(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
        ValidatedJson(dto): ValidatedJson<UpdateUserRequestDto>,
    ) -> ApiResult<UserResponseDto> {
        let user = Service::update_user(&state.pool, id, &dto).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Usuário atualizado com sucesso.",
                Some(UserResponseDto::from(user)),
            ),
        ))
    }

    pub async fn delete_user(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<()> {
        Service::delete_user(&state.pool, id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Usuário deletado com sucesso.", None),
        ))
    }
}

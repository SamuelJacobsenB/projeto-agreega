use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    response::{ApiResponse, ApiResult, AppResult, ValidatedJson, ValidatedMultipart},
    security::auth_user::AuthUser,
};

use super::{
    dtos::{CreatePhotoRequestDto, PhotoResponseDto, UpdatePhotoDescriptionRequestDto},
    service::Service,
};

pub struct Handler;

impl Handler {
    pub async fn list_project_photos(
        State(state): State<AppState>,
        AuthUser { id, role }: AuthUser,
        Path(project_id): Path<Uuid>,
    ) -> ApiResult<Vec<PhotoResponseDto>> {
        let photos = Service::list_project_photos(&state.pool, project_id, id, role).await?;

        let photo_dtos = photos
            .into_iter()
            .map(PhotoResponseDto::from_model)
            .collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Fotos do projeto encontradas.", Some(photo_dtos)),
        ))
    }

    pub async fn list_stage_photos(
        State(state): State<AppState>,
        AuthUser { id, role }: AuthUser,
        Path(stage_id): Path<Uuid>,
    ) -> ApiResult<Vec<PhotoResponseDto>> {
        let photos = Service::list_stage_photos(&state.pool, stage_id, id, role).await?;

        let photo_dtos = photos
            .into_iter()
            .map(PhotoResponseDto::from_model)
            .collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Fotos da etapa encontradas.", Some(photo_dtos)),
        ))
    }

    pub async fn list_task_photos(
        State(state): State<AppState>,
        AuthUser { id, role }: AuthUser,
        Path(task_id): Path<Uuid>,
    ) -> ApiResult<Vec<PhotoResponseDto>> {
        let photos = Service::list_task_photos(&state.pool, task_id, id, role).await?;

        let photo_dtos = photos
            .into_iter()
            .map(PhotoResponseDto::from_model)
            .collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Fotos da tarefa encontradas.", Some(photo_dtos)),
        ))
    }

    pub async fn get_photo_by_id(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(photo_id): Path<Uuid>,
    ) -> ApiResult<PhotoResponseDto> {
        let photo = Service::get_photo_by_id(&state.pool, photo_id, user_id, role).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Foto encontrada.",
                Some(PhotoResponseDto::from_model(photo)),
            ),
        ))
    }

    pub async fn create_photo(
        State(state): State<AppState>,
        AuthUser {
            id: uploaded_by,
            role,
        }: AuthUser,
        ValidatedMultipart { data, file }: ValidatedMultipart<CreatePhotoRequestDto>,
    ) -> ApiResult<PhotoResponseDto> {
        let photo = Service::create_photo(&state.pool, &data, file, uploaded_by, role).await?;
        Ok((
            StatusCode::CREATED,
            ApiResponse::success(
                "Foto criada com sucesso.",
                Some(PhotoResponseDto::from_model(photo)),
            ),
        ))
    }

    pub async fn update_description(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
        ValidatedJson(dto): ValidatedJson<UpdatePhotoDescriptionRequestDto>,
    ) -> ApiResult<PhotoResponseDto> {
        let photo = Service::update_description(&state.pool, id, &dto).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Descrição da foto atualizada.",
                Some(PhotoResponseDto::from_model(photo)),
            ),
        ))
    }

    pub async fn delete_photo(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> ApiResult<()> {
        Service::delete_photo(&state.pool, id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Foto excluída com sucesso.", None),
        ))
    }

    pub async fn get_file(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(id): Path<Uuid>,
    ) -> AppResult<Response> {
        Service::get_file(&state.pool, id, user_id, role).await
    }
}

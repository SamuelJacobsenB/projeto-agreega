use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    domains::project_images::{
        dtos::{
            CreateProjectImageRequestDto, ProjectImageResponseDto, ReorderProjectImagesRequestDto,
        },
        service::Service,
    },
    response::{ApiResponse, ApiResult, AppResult, ValidatedJson, ValidatedMultipart},
    security::auth_user::AuthUser,
};

pub struct Handler;

impl Handler {
    pub async fn list_project_images(
        State(state): State<AppState>,
        Path(project_id): Path<Uuid>,
    ) -> ApiResult<Vec<ProjectImageResponseDto>> {
        let project_images = Service::list_project_images(&state.pool, project_id).await?;

        let project_image_dtos = project_images
            .into_iter()
            .map(ProjectImageResponseDto::from_model)
            .collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Imagens do projeto encontradas.", Some(project_image_dtos)),
        ))
    }

    pub async fn get_cover_by_project_id(
        State(state): State<AppState>,
        Path(project_id): Path<Uuid>,
    ) -> ApiResult<ProjectImageResponseDto> {
        let cover_image = Service::get_cover_by_project_id(&state.pool, project_id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Capa do projeto encontrada.",
                Some(ProjectImageResponseDto::from_model(cover_image)),
            ),
        ))
    }

    pub async fn get_project_image_by_id(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> ApiResult<ProjectImageResponseDto> {
        let project_image = Service::get_project_image_by_id(&state.pool, id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Imagem do projeto encontrada.",
                Some(ProjectImageResponseDto::from_model(project_image)),
            ),
        ))
    }

    pub async fn create_project_image(
        State(state): State<AppState>,
        AuthUser {
            id: uploaded_by, ..
        }: AuthUser,
        ValidatedMultipart { data: dto, file }: ValidatedMultipart<CreateProjectImageRequestDto>,
    ) -> ApiResult<ProjectImageResponseDto> {
        let project_image =
            Service::create_project_image(&state.pool, &dto, file, uploaded_by).await?;

        Ok((
            StatusCode::CREATED,
            ApiResponse::success(
                "Imagem do projeto criada com sucesso.",
                Some(ProjectImageResponseDto::from_model(project_image)),
            ),
        ))
    }

    pub async fn reorder_project_images(
        State(state): State<AppState>,
        Path(project_id): Path<Uuid>,
        ValidatedJson(dto): ValidatedJson<ReorderProjectImagesRequestDto>,
    ) -> ApiResult<()> {
        Service::reorder_project_images(&state.pool, project_id, &dto).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Ordem das imagens alterada com sucesso.", None),
        ))
    }

    pub async fn delete_project_image(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> ApiResult<()> {
        Service::delete_project_image(&state.pool, id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Imagem do projeto excluída com sucesso.", None),
        ))
    }

    pub async fn get_file(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> AppResult<Response> {
        Service::get_file(&state.pool, id).await
    }
}

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    domains::projects::{
        dtos::{CreateProjectRequestDto, ProjectResponseDto, UpdateProjectRequestDto},
        service::Service,
    },
    response::{ApiResponse, ApiResult, ValidatedJson},
};

pub struct Handler;

impl Handler {
    pub async fn list_projects(
        State(state): State<AppState>,
    ) -> ApiResult<Vec<ProjectResponseDto>> {
        let projects = Service::list_projects(&state.pool).await?;

        let project_dtos = projects.into_iter().map(ProjectResponseDto::from).collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Projetos encontrados.", Some(project_dtos)),
        ))
    }

    pub async fn get_project_by_id(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> ApiResult<ProjectResponseDto> {
        let project = Service::get_project_by_id(&state.pool, id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Projeto encontrado.",
                Some(ProjectResponseDto::from(project)),
            ),
        ))
    }

    pub async fn create_project(
        State(state): State<AppState>,
        ValidatedJson(dto): ValidatedJson<CreateProjectRequestDto>,
    ) -> ApiResult<ProjectResponseDto> {
        let project = Service::create_project(&state.pool, &dto).await?;

        Ok((
            StatusCode::CREATED,
            ApiResponse::success(
                "Projeto criado com sucesso.",
                Some(ProjectResponseDto::from(project)),
            ),
        ))
    }

    pub async fn update_project(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
        ValidatedJson(dto): ValidatedJson<UpdateProjectRequestDto>,
    ) -> ApiResult<ProjectResponseDto> {
        let project = Service::update_project(&state.pool, id, &dto).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Projeto atualizado com sucesso.",
                Some(ProjectResponseDto::from(project)),
            ),
        ))
    }

    pub async fn delete_project(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> ApiResult<()> {
        Service::delete_project(&state.pool, id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Projeto excluído com sucesso.", None),
        ))
    }
}

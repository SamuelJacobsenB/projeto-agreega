use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    domains::stages::{
        dtos::{CreateStageRequestDto, ReorderStagesRequestDto, StageResponseDto},
        service::Service,
    },
    response::{ApiResponse, ApiResult, ValidatedJson},
    security::auth_user::AuthUser,
};

pub struct Handler;

impl Handler {
    pub async fn list_project_stages(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(project_id): Path<Uuid>,
    ) -> ApiResult<Vec<StageResponseDto>> {
        let stages = Service::list_project_stages(&state.pool, project_id, user_id, role).await?;

        let stage_dtos = stages.into_iter().map(StageResponseDto::from).collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Estágios do projeto encontrados.", Some(stage_dtos)),
        ))
    }

    pub async fn get_stage_by_id(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(id): Path<Uuid>,
    ) -> ApiResult<StageResponseDto> {
        let stage = Service::get_stage_by_id(&state.pool, id, user_id, role).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Estágio encontrado com sucesso.",
                Some(StageResponseDto::from(stage)),
            ),
        ))
    }

    pub async fn create_stage(
        State(state): State<AppState>,
        ValidatedJson(dto): ValidatedJson<CreateStageRequestDto>,
    ) -> ApiResult<StageResponseDto> {
        let stage = Service::create_stage(&state.pool, &dto).await?;

        Ok((
            StatusCode::CREATED,
            ApiResponse::success(
                "Estágio do projeto criado com sucesso.",
                Some(StageResponseDto::from(stage)),
            ),
        ))
    }

    pub async fn reorder_stages(
        State(state): State<AppState>,
        Path(project_id): Path<Uuid>,
        ValidatedJson(dto): ValidatedJson<ReorderStagesRequestDto>,
    ) -> ApiResult<()> {
        Service::reorder_stages(&state.pool, project_id, &dto).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Ordem dos estágios alterada com sucesso.", None),
        ))
    }

    pub async fn delete_stage(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> ApiResult<()> {
        Service::delete_stage(&state.pool, id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Estágio do projeto excluído com sucesso.", None),
        ))
    }
}

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    domains::tasks::{
        dtos::{CreateTaskRequestDto, ReorderTasksRequestDto, TaskResponseDto},
        service::Service,
    },
    response::{ApiResponse, ApiResult, ValidatedJson},
};

pub struct Handler;

impl Handler {
    pub async fn list_stage_tasks(
        State(state): State<AppState>,
        Path(stage_id): Path<Uuid>,
    ) -> ApiResult<Vec<TaskResponseDto>> {
        let tasks = Service::list_stage_tasks(&state.pool, stage_id).await?;

        let task_dtos = tasks.into_iter().map(TaskResponseDto::from).collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Tarefas do estágio encontradas.", Some(task_dtos)),
        ))
    }

    pub async fn get_task_by_id(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> ApiResult<TaskResponseDto> {
        let task = Service::get_task_by_id(&state.pool, id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Tarefa encontrada com sucesso.",
                Some(TaskResponseDto::from(task)),
            ),
        ))
    }

    pub async fn create_task(
        State(state): State<AppState>,
        ValidatedJson(dto): ValidatedJson<CreateTaskRequestDto>,
    ) -> ApiResult<TaskResponseDto> {
        let task = Service::create_task(&state.pool, &dto).await?;

        Ok((
            StatusCode::CREATED,
            ApiResponse::success(
                "Tarefa do estágio criada com sucesso.",
                Some(TaskResponseDto::from(task)),
            ),
        ))
    }

    pub async fn reorder_tasks(
        State(state): State<AppState>,
        Path(stage_id): Path<Uuid>,
        ValidatedJson(dto): ValidatedJson<ReorderTasksRequestDto>,
    ) -> ApiResult<()> {
        Service::reorder_tasks(&state.pool, stage_id, &dto).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Ordem das tarefas alterada com sucesso.", None),
        ))
    }

    pub async fn delete_task(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<()> {
        Service::delete_task(&state.pool, id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Tarefa do projeto excluída com sucesso.", None),
        ))
    }
}

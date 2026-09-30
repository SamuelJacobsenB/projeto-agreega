use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    domains::clients::{
        dtos::{ClientResponseDto, CreateClientRequestDto, UpdateClientRequestDto},
        service::Service,
    },
    response::{ApiResponse, ApiResult, ValidatedJson},
};

pub struct Handler;

impl Handler {
    pub async fn get_all_clients(
        State(state): State<AppState>,
    ) -> ApiResult<Vec<ClientResponseDto>> {
        let clients = Service::get_all_clients(&state.pool).await?;

        let client_dtos = clients.into_iter().map(ClientResponseDto::from).collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Clientes encontrados.", Some(client_dtos)),
        ))
    }

    pub async fn get_client_by_id(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> ApiResult<ClientResponseDto> {
        let client = Service::get_client_by_id(&state.pool, id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Cliente encontrado.", Some(ClientResponseDto::from(client))),
        ))
    }

    pub async fn create_client(
        State(state): State<AppState>,
        ValidatedJson(dto): ValidatedJson<CreateClientRequestDto>,
    ) -> ApiResult<ClientResponseDto> {
        let client = Service::create_client(&state.pool, &dto).await?;

        Ok((
            StatusCode::CREATED,
            ApiResponse::success(
                "Cliente criado com sucesso.",
                Some(ClientResponseDto::from(client)),
            ),
        ))
    }

    pub async fn update_client(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
        ValidatedJson(dto): ValidatedJson<UpdateClientRequestDto>,
    ) -> ApiResult<ClientResponseDto> {
        let client = Service::update_client(&state.pool, id, &dto).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Cliente atualizado com sucesso.",
                Some(ClientResponseDto::from(client)),
            ),
        ))
    }

    pub async fn delete_client(
        State(state): State<AppState>,
        Path(id): Path<Uuid>,
    ) -> ApiResult<()> {
        Service::delete_client(&state.pool, id).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success("Cliente excluído com sucesso.", None),
        ))
    }
}

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    app::AppState,
    domains::payments::{
        dtos::{CreatePaymentRequestDto, PaymentResponseDto, UpdatePaymentRequestDto},
        service::Service,
    },
    response::{ApiResponse, ApiResult, ValidatedJson},
    security::auth_user::AuthUser,
};

pub struct Handler;

impl Handler {
    pub async fn list_by_project_id(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(project_id): Path<Uuid>,
    ) -> ApiResult<Vec<PaymentResponseDto>> {
        let payments = Service::list_by_project_id(&state.pool, project_id, user_id, role).await?;

        let payment_dtos = payments.into_iter().map(PaymentResponseDto::from).collect();

        Ok((
            StatusCode::OK,
            ApiResponse::success("Pagamentos do projeto encontrados.", Some(payment_dtos)),
        ))
    }

    pub async fn create(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        ValidatedJson(dto): ValidatedJson<CreatePaymentRequestDto>,
    ) -> ApiResult<PaymentResponseDto> {
        let payment = Service::create(&state.pool, &dto, user_id, role).await?;

        Ok((
            StatusCode::CREATED,
            ApiResponse::success(
                "Pagamento criado com sucesso.",
                Some(PaymentResponseDto::from(payment)),
            ),
        ))
    }

    pub async fn update(
        State(state): State<AppState>,
        AuthUser { id: user_id, role }: AuthUser,
        Path(id): Path<Uuid>,
        ValidatedJson(dto): ValidatedJson<UpdatePaymentRequestDto>,
    ) -> ApiResult<PaymentResponseDto> {
        let payment = Service::update(&state.pool, id, &dto, user_id, role).await?;

        Ok((
            StatusCode::OK,
            ApiResponse::success(
                "Pagamento atualizado com sucesso.",
                Some(PaymentResponseDto::from(payment)),
            ),
        ))
    }
}

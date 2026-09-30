use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::collections::BTreeMap;

use super::ApiResponse;

pub enum AppError {
    Database(String),
    NotFound(String),
    Conflict(String),
    Validation(BTreeMap<String, Vec<String>>),
    Unauthorized(String),
    Forbidden(String),
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, response) = match self {
            Self::NotFound(message) => (StatusCode::NOT_FOUND, ApiResponse::<()>::error(message)),

            Self::Conflict(message) => (StatusCode::CONFLICT, ApiResponse::<()>::error(message)),

            Self::Validation(fields) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ApiResponse::<()>::validation("Dados inválidos.", fields),
            ),

            Self::BadRequest(message) => {
                (StatusCode::BAD_REQUEST, ApiResponse::<()>::error(message))
            }

            Self::Unauthorized(message) => {
                (StatusCode::UNAUTHORIZED, ApiResponse::<()>::error(message))
            }

            Self::Forbidden(message) => (StatusCode::FORBIDDEN, ApiResponse::<()>::error(message)),

            Self::Database(_) | Self::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiResponse::<()>::error("Erro interno ao processar a solicitação."),
            ),
        };

        (status, Json(response)).into_response()
    }
}

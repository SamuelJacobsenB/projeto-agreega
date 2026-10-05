use std::collections::BTreeMap;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::response::AppError;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<BTreeMap<String, Vec<String>>>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl<T> ApiResponse<T> {
    pub fn success(message: impl Into<String>, data: Option<T>) -> Self {
        Self {
            success: true,
            message: message.into(),
            data,
            fields: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            data: None,
            fields: None,
        }
    }

    pub fn validation(message: impl Into<String>, fields: BTreeMap<String, Vec<String>>) -> Self {
        Self {
            success: false,
            message: message.into(),
            data: None,
            fields: Some(fields),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
pub type ApiResult<T> = Result<(StatusCode, ApiResponse<T>), AppError>;

use axum::{
    Json,
    extract::{FromRequest, Request},
    http::StatusCode,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use super::ApiResponse;

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate + Send,
{
    type Rejection = (StatusCode, Json<ApiResponse<()>>);

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(request, state).await.map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("Corpo da requisição inválido.")),
            )
        })?;

        value.validate().map_err(|errors| {
            let fields = errors
                .field_errors()
                .into_iter()
                .map(|(field, errors)| {
                    let messages = errors
                        .iter()
                        .filter_map(|error| error.message.as_deref())
                        .map(String::from)
                        .collect();

                    (field.to_string(), messages)
                })
                .collect();

            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponse::validation("Dados inválidos.", fields)),
            )
        })?;

        Ok(Self(value))
    }
}

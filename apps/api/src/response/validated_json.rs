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
            let mut messages = errors
                .field_errors()
                .iter()
                .flat_map(|(field, field_errors)| {
                    field_errors.iter().map(move |error| {
                        let message = error.message.as_deref().unwrap_or("Valor inválido.");
                        format!("{field}: {message}")
                    })
                })
                .collect::<Vec<_>>();

            messages.sort();
            let message = if messages.is_empty() {
                "Dados inválidos.".to_owned()
            } else {
                messages.join(" ")
            };

            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ApiResponse::error(message)),
            )
        })?;

        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        Json,
        body::Body,
        extract::FromRequest,
        http::{Request, StatusCode, header::CONTENT_TYPE},
    };

    use crate::domains::auth::dtos::LoginRequestDto;

    use super::ValidatedJson;

    #[tokio::test]
    async fn invalid_json_fields_are_rejected_without_echoing_values() {
        let request = Request::builder()
            .method("POST")
            .uri("/")
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(
                r#"{"email":"private-email-value","password":"private-password-value"}"#,
            ))
            .unwrap();

        let result =
            <ValidatedJson<LoginRequestDto> as FromRequest<()>>::from_request(request, &()).await;
        let (status, Json(response)) = match result {
            Ok(_) => panic!("invalid email should be rejected"),
            Err(rejection) => rejection,
        };

        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        assert!(!response.message.contains("private-email-value"));
        assert!(!response.message.contains("private-password-value"));
    }
}

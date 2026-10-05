use axum::{
    Json,
    extract::{FromRequest, Multipart, Request},
    http::StatusCode,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use super::ApiResponse;
use crate::domains::files::types::UploadedFile;

pub struct ValidatedMultipart<T> {
    pub data: T,
    pub file: UploadedFile,
}

impl<S, T> FromRequest<S> for ValidatedMultipart<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate + Send,
{
    type Rejection = (StatusCode, Json<ApiResponse<()>>);

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let mut multipart = Multipart::from_request(request, state).await.map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("Corpo da requisição inválido.")),
            )
        })?;

        let mut data: Option<T> = None;
        let mut file: Option<UploadedFile> = None;

        while let Some(field) = multipart.next_field().await.map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("Falha ao ler os dados da requisição.")),
            )
        })? {
            let name = field.name().map(str::to_owned);

            match name.as_deref() {
                Some("data") => {
                    if data.is_some() {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            Json(ApiResponse::error(
                                "O campo 'data' foi enviado mais de uma vez.",
                            )),
                        ));
                    }

                    let bytes = field.bytes().await.map_err(|_| {
                        (
                            StatusCode::BAD_REQUEST,
                            Json(ApiResponse::error("Falha ao ler os dados.")),
                        )
                    })?;

                    let value = serde_json::from_slice::<T>(&bytes).map_err(|_| {
                        (
                            StatusCode::BAD_REQUEST,
                            Json(ApiResponse::error("Os dados enviados são inválidos.")),
                        )
                    })?;

                    let errors = match value.validate() {
                        Ok(()) => None,
                        Err(errors) => {
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

                            Some(fields)
                        }
                    };

                    if let Some(fields) = errors {
                        return Err((
                            StatusCode::UNPROCESSABLE_ENTITY,
                            Json(ApiResponse::validation("Dados inválidos.", fields)),
                        ));
                    }

                    data = Some(value);
                }

                Some("file") => {
                    if file.is_some() {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            Json(ApiResponse::error(
                                "O campo 'file' foi enviado mais de uma vez.",
                            )),
                        ));
                    }

                    let original_name = field.file_name().map(String::from).ok_or_else(|| {
                        (
                            StatusCode::BAD_REQUEST,
                            Json(ApiResponse::error("O arquivo não possui um nome válido.")),
                        )
                    })?;

                    let content = field.bytes().await.map_err(|_| {
                        (
                            StatusCode::BAD_REQUEST,
                            Json(ApiResponse::error("Falha ao ler o arquivo.")),
                        )
                    })?;

                    file = Some(UploadedFile {
                        original_name,
                        content,
                    });
                }

                Some(_) | None => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse::error("Campo não permitido na requisição.")),
                    ));
                }
            }
        }

        let data = data.ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("O campo 'data' é obrigatório.")),
            )
        })?;

        let file = file.ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error("O campo 'file' é obrigatório.")),
            )
        })?;

        Ok(Self { data, file })
    }
}

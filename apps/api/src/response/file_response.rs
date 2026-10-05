use axum::{
    body::Body,
    http::{
        HeaderValue,
        header::{CACHE_CONTROL, CONTENT_LENGTH, CONTENT_TYPE},
    },
    response::Response,
};

use crate::response::{AppError, AppResult};

pub fn file(content: Vec<u8>, mime_type: &str) -> AppResult<Response> {
    let content_type = HeaderValue::from_str(mime_type)
        .map_err(|_| AppError::Internal("Tipo de arquivo inválido.".to_string()))?;

    let content_length = HeaderValue::from_str(&content.len().to_string())
        .map_err(|_| AppError::Internal("Tamanho de arquivo inválido.".to_string()))?;

    let mut response = Response::new(Body::from(content));

    response.headers_mut().insert(CONTENT_TYPE, content_type);

    response
        .headers_mut()
        .insert(CONTENT_LENGTH, content_length);

    response.headers_mut().insert(
        CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=31536000, immutable"),
    );

    Ok(response)
}

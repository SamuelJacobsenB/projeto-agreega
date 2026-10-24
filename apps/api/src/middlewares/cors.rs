use std::time::Duration;

use axum::http::HeaderValue;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE, InvalidHeaderValue};
use resend_rs::Method;
use tower_http::cors::CorsLayer;

pub fn cors_layer(allowed_urls: [&str; 2]) -> Result<CorsLayer, InvalidHeaderValue> {
    let origins = allowed_urls
        .iter()
        .map(|url| HeaderValue::try_from(*url))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600)))
}

use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};

use crate::{
    app::AppState,
    domains::users::models::UserRole,
    response::{AppError, AppResult},
    security::jwt::{AccessClaims, JwtService},
};

pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> AppResult<Response> {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            let (scheme, token) = header.split_once(' ')?;

            scheme.eq_ignore_ascii_case("Bearer").then_some(token)
        })
        .filter(|token| !token.is_empty())
        .ok_or_else(|| AppError::Unauthorized("Usuário não autenticado.".to_string()))?;

    let claims = JwtService::decode_access_token(token, &state.config.jwt_secret)?;

    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

pub async fn require_staff(request: Request, next: Next) -> AppResult<Response> {
    let claims = get_claims(&request)?;

    if matches!(claims.role, UserRole::Client) {
        return Err(AppError::Forbidden("Acesso restrito.".to_string()));
    }

    Ok(next.run(request).await)
}

pub async fn require_admin(request: Request, next: Next) -> AppResult<Response> {
    let claims = get_claims(&request)?;

    if !matches!(claims.role, UserRole::Admin) {
        return Err(AppError::Forbidden("Acesso restrito.".to_string()));
    }

    Ok(next.run(request).await)
}

fn get_claims(request: &Request) -> AppResult<&AccessClaims> {
    request
        .extensions()
        .get::<AccessClaims>()
        .ok_or_else(|| AppError::Unauthorized("Usuário não autenticado.".to_string()))
}

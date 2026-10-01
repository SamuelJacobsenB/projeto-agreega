use axum::{extract::FromRequestParts, http::request::Parts};
use uuid::Uuid;

use crate::{domains::users::models::UserRole, response::AppError, security::jwt::AccessClaims};

pub struct AuthUser {
    pub id: Uuid,
    pub role: UserRole,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parts
            .extensions
            .get::<AccessClaims>()
            .ok_or_else(|| AppError::Unauthorized("Usuário não autenticado.".to_string()))?;

        let id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("Identidade do usuário inválida.".to_string()))?;

        Ok(Self {
            id,
            role: claims.role.clone(),
        })
    }
}

use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{
    domains::users::models::{User, UserRole},
    response::{AppError, AppResult},
};

const ACCESS_TOKEN_MINUTES: i64 = 15;

#[derive(Clone, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String,
    pub role: UserRole,
    pub exp: usize,
}

pub struct JwtService;

impl JwtService {
    pub fn create_access_token(
        user: &User,
        jwt_secret: &str,
    ) -> AppResult<(String, DateTime<Utc>)> {
        let expires_at = Utc::now() + Duration::minutes(ACCESS_TOKEN_MINUTES);
        let claims = AccessClaims {
            sub: user.id.to_string(),
            role: user.role.clone(),
            exp: expires_at.timestamp() as usize,
        };

        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        )
        .map_err(|_| AppError::Internal("Falha ao gerar token de acesso.".to_string()))?;

        Ok((token, expires_at))
    }

    pub fn decode_access_token(token: &str, jwt_secret: &str) -> AppResult<AccessClaims> {
        let token_data = jsonwebtoken::decode::<AccessClaims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(jwt_secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized("Token de acesso inválido ou expirado.".to_string()))?;

        Ok(token_data.claims)
    }
}

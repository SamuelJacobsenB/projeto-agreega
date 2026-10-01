use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};

use crate::response::{AppError, AppResult};

pub struct PasswordService;

impl PasswordService {
    pub fn hash_password(password: &str) -> AppResult<String> {
        Argon2::default()
            .hash_password(password.as_bytes())
            .map(|hash| hash.to_string())
            .map_err(|_| AppError::Internal("Falha ao proteger senha.".to_string()))
    }

    pub fn verify_password(password: &str, password_hash: &str) -> AppResult<()> {
        let hash = PasswordHash::new(password_hash)
            .map_err(|_| AppError::Internal("Hash de senha inválido.".to_string()))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .map_err(|_| AppError::Unauthorized("E-mail ou senha inválidos.".to_string()))
    }
}

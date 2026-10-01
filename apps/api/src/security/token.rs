use getrandom::fill;
use sha2::{Digest, Sha256};

use crate::response::{AppError, AppResult};

pub struct TokenService;

impl TokenService {
    pub fn new_token() -> AppResult<String> {
        let mut bytes = [0u8; 32];

        fill(&mut bytes)
            .map_err(|_| AppError::Internal("Falha ao gerar token seguro.".to_string()))?;

        Ok(hex::encode(bytes))
    }

    pub fn hash_token(token: &str) -> String {
        hex::encode(Sha256::digest(token.as_bytes()))
    }
}

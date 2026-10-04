use std::path::{Component, Path, PathBuf};

use tokio::fs;

use crate::response::{AppError, AppResult};

pub struct StorageService;

impl StorageService {
    const STORAGE_DIR: &'static str = "storage";

    pub async fn file_exists(storage_key: &str) -> AppResult<bool> {
        let path = Self::build_path(storage_key)?;

        fs::try_exists(path)
            .await
            .map_err(|_| AppError::Internal("Erro ao verificar arquivo.".to_string()))
    }

    pub async fn read_file(storage_key: &str) -> AppResult<Vec<u8>> {
        let path = Self::build_path(storage_key)?;

        fs::read(path)
            .await
            .map_err(|_| AppError::Internal("Erro ao ler arquivo.".to_string()))
    }

    pub async fn upload_file(storage_key: &str, content: &[u8]) -> AppResult<()> {
        let path = Self::build_path(storage_key)?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await.map_err(|_| {
                AppError::Internal("Erro ao criar diretório do arquivo.".to_string())
            })?;
        }

        fs::write(path, content)
            .await
            .map_err(|_| AppError::Internal("Erro ao salvar arquivo.".to_string()))?;

        Ok(())
    }

    pub async fn delete_file(storage_key: &str) -> AppResult<()> {
        let path = Self::build_path(storage_key)?;

        fs::remove_file(path)
            .await
            .map_err(|_| AppError::Internal("Erro ao excluir arquivo.".to_string()))?;

        Ok(())
    }

    fn build_path(storage_key: &str) -> AppResult<PathBuf> {
        let path = Path::new(storage_key);

        if storage_key.is_empty()
            || path.is_absolute()
            || path.components().any(|c| c == Component::ParentDir)
        {
            return Err(AppError::BadRequest("Storage key inválida.".to_string()));
        }

        Ok(PathBuf::from(Self::STORAGE_DIR).join(path))
    }
}

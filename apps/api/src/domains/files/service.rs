use sha2::{Digest, Sha256};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::files::{models::File, repository::Repository},
    infrastructure::storage::StorageService,
    response::{AppError, AppResult},
};

pub struct Service;

impl Service {
    pub async fn get_file_by_id(pool: &PgPool, id: Uuid) -> AppResult<File> {
        Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Arquivo não encontrado.".to_string()))
    }

    pub async fn create_file(
        pool: &PgPool,
        storage_prefix: &str,
        original_name: &str,
        content: &[u8],
        allowed_mime_types: &[&str],
        uploaded_by: Uuid,
        max_file_size_bytes: i64,
    ) -> AppResult<File> {
        let mime_type = infer::get(content)
            .map(|kind| kind.mime_type().to_string())
            .ok_or_else(|| {
                AppError::BadRequest("Não foi possível identificar o tipo do arquivo.".to_string())
            })?;

        let size_bytes = content.len() as i64;
        if size_bytes > max_file_size_bytes {
            return Err(AppError::BadRequest(
                "O arquivo excede o tamanho máximo permitido.".to_string(),
            ));
        }

        if !allowed_mime_types.contains(&mime_type.as_str()) {
            return Err(AppError::BadRequest(
                "Tipo de arquivo não permitido.".to_string(),
            ));
        }

        let checksum = Sha256::digest(content)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();

        let id = Uuid::new_v4();

        let storage_key = format!("{storage_prefix}/{id}");

        StorageService::upload_file(&storage_key, content).await?;

        match Repository::create(
            pool,
            id,
            &storage_key,
            original_name,
            &mime_type,
            size_bytes,
            &checksum,
            uploaded_by,
        )
        .await
        {
            Ok(file) => Ok(file),
            Err(error) => {
                let _ = StorageService::delete_file(&storage_key).await;
                Err(error)
            }
        }
    }

    pub async fn delete_file(pool: &PgPool, id: Uuid) -> AppResult<()> {
        let file = Self::get_file_by_id(pool, id).await?;

        StorageService::delete_file(&file.storage_key).await?;

        Repository::delete(pool, id).await?;

        Ok(())
    }
}

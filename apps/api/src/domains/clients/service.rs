use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::clients::{
        dtos::{CreateClientRequestDto, UpdateClientRequestDto},
        models::Client,
        repository::Repository,
    },
    response::{AppError, AppResult},
};

pub struct Service;

impl Service {
    pub async fn list_clients(pool: &PgPool) -> AppResult<Vec<Client>> {
        Repository::find_all(pool).await
    }

    pub async fn get_client_by_id(pool: &PgPool, id: Uuid) -> AppResult<Client> {
        Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Cliente não encontrado.".to_string()))
    }

    pub async fn get_my_client(pool: &PgPool, user_id: Uuid) -> AppResult<Client> {
        Repository::find_client_by_user_id(pool, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Cliente não encontrado.".to_string()))
    }

    pub async fn create_client(pool: &PgPool, dto: &CreateClientRequestDto) -> AppResult<Client> {
        if let Some(cnpj) = &dto.cnpj {
            if Repository::exists_by_cnpj(pool, cnpj, None).await? {
                return Err(AppError::Conflict(
                    "Este CNPJ já foi cadastrado.".to_string(),
                ));
            }
        }

        if let Some(email) = &dto.email {
            if Repository::exists_by_email(pool, email, None).await? {
                return Err(AppError::Conflict(
                    "Este e-mail já foi cadastrado.".to_string(),
                ));
            }
        }

        Repository::create(pool, dto).await
    }

    pub async fn update_client(
        pool: &PgPool,
        id: Uuid,
        dto: &UpdateClientRequestDto,
    ) -> AppResult<Client> {
        if let Some(cnpj) = &dto.cnpj {
            if Repository::exists_by_cnpj(pool, cnpj, Some(id)).await? {
                return Err(AppError::Conflict(
                    "Este CNPJ já foi cadastrado.".to_string(),
                ));
            }
        }

        if let Some(email) = &dto.email {
            if Repository::exists_by_email(pool, email, Some(id)).await? {
                return Err(AppError::Conflict(
                    "Este e-mail já foi cadastrado.".to_string(),
                ));
            }
        }

        Repository::update(pool, id, dto).await
    }

    pub async fn delete_client(pool: &PgPool, id: Uuid) -> AppResult<()> {
        Repository::delete(pool, id).await
    }
}

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::users::{dtos::UpdateUserRequestDto, models::User, repository::Repository},
    response::{AppError, AppResult},
};

pub struct Service;

impl Service {
    pub async fn get_all_users(pool: &PgPool) -> AppResult<Vec<User>> {
        Repository::find_all(pool).await
    }

    pub async fn get_staff_users(pool: &PgPool) -> AppResult<Vec<User>> {
        Repository::find_staff(pool).await
    }

    pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> AppResult<User> {
        Repository::find_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Usuário não encontrado.".to_string()))
    }

    pub async fn update_user(
        pool: &PgPool,
        id: Uuid,
        dto: &UpdateUserRequestDto,
    ) -> AppResult<User> {
        if let Some(phone) = &dto.phone {
            if let Some(existing_user) = Repository::find_by_phone(pool, phone).await? {
                if existing_user.id != id {
                    return Err(crate::response::AppError::Conflict(
                        "Telefone já está em uso por outro usuário".to_string(),
                    ));
                }
            }
        }

        if let Some(cpf) = &dto.cpf {
            if let Some(existing_user) = Repository::find_by_cpf(pool, cpf).await? {
                if existing_user.id != id {
                    return Err(crate::response::AppError::Conflict(
                        "CPF já está em uso por outro usuário".to_string(),
                    ));
                }
            }
        }

        Repository::update(pool, id, dto)
            .await?
            .ok_or_else(|| AppError::NotFound("Usuário não encontrado.".to_string()))
    }

    pub async fn delete_user(pool: &PgPool, id: Uuid) -> AppResult<()> {
        Repository::delete(pool, id).await
    }
}

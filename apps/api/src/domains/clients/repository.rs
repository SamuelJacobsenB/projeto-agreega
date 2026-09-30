use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::clients::{
        dtos::{CreateClientRequestDto, UpdateClientRequestDto},
        models::Client,
    },
    response::{AppError, AppResult},
};

pub struct Repository;

impl Repository {
    pub async fn exists_by_cnpj(
        pool: &PgPool,
        cnpj: &str,
        except_id: Option<Uuid>,
    ) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM clients
                WHERE cnpj = $1
                    AND ($2::uuid IS NULL OR id <> $2)
            ) AS "exists!"
            "#,
            cnpj,
            except_id
        )
        .fetch_one(pool)
        .await
        .map_err(|_| {
            AppError::Database("Falha ao verificar existência de cliente nos dados.".to_string())
        })?;

        Ok(result.exists)
    }

    pub async fn exists_by_email(
        pool: &PgPool,
        email: &str,
        except_id: Option<Uuid>,
    ) -> AppResult<bool> {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM clients
                WHERE email = $1
                    AND ($2::uuid IS NULL OR id <> $2)
            ) AS "exists!"
            "#,
            email,
            except_id
        )
        .fetch_one(pool)
        .await
        .map_err(|_| {
            AppError::Database("Falha ao verificar existência de cliente nos dados.".to_string())
        })?;

        Ok(result.exists)
    }

    pub async fn find_all(pool: &PgPool) -> AppResult<Vec<Client>> {
        sqlx::query_as!(
            Client,
            r#"
            SELECT *
            FROM clients
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar clientes nos dados.".to_string()))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<Client>> {
        sqlx::query_as!(
            Client,
            r#"
            SELECT *
            FROM clients
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar cliente nos dados.".to_string()))
    }

    pub async fn create(pool: &PgPool, dto: &CreateClientRequestDto) -> AppResult<Client> {
        sqlx::query_as!(
            Client,
            r#"
            INSERT INTO clients (id, company_name, cnpj, phone, email, address, city, state)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
            Uuid::new_v4(),
            dto.company_name,
            dto.cnpj,
            dto.phone,
            dto.email,
            dto.address,
            dto.city,
            dto.state
        )
        .fetch_one(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao criar cliente nos dados.".to_string()))
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        dto: &UpdateClientRequestDto,
    ) -> AppResult<Client> {
        sqlx::query_as!(
            Client,
            r#"
            UPDATE clients
            SET company_name = COALESCE($1, company_name),
                cnpj = COALESCE($2, cnpj),
                phone = COALESCE($3, phone),
                email = COALESCE($4, email),
                address = COALESCE($5, address),
                city = COALESCE($6, city),
                state = COALESCE($7, state),
                updated_at = NOW()
            WHERE id = $8
            RETURNING *
            "#,
            dto.company_name,
            dto.cnpj,
            dto.phone,
            dto.email,
            dto.address,
            dto.city,
            dto.state,
            id
        )
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao atualizar cliente nos dados.".to_string()))?
        .ok_or_else(|| AppError::NotFound("Cliente não encontrado.".to_string()))
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> AppResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM clients
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao deletar cliente nos dados.".to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Cliente não encontrado.".to_string()));
        }

        Ok(())
    }
}

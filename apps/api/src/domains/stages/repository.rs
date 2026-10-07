use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::stages::{
        dtos::{CreateStageRequestDto, ReorderStagesRequestDto},
        models::Stage,
    },
    infrastructure::db::ordering::{self, OrderingTable},
    response::{AppError, AppResult},
};

pub struct Repository;

impl Repository {
    pub async fn find_by_project_id(pool: &PgPool, project_id: Uuid) -> AppResult<Vec<Stage>> {
        sqlx::query_as::<_, Stage>(
            r#"
                SELECT *
                FROM stages
                WHERE project_id = $1
            "#,
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar estágios nos dados.".to_string()))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<Stage>> {
        sqlx::query_as::<_, Stage>(
            r#"
                SELECT *
                FROM stages
                WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar estágio nos dados.".to_string()))
    }

    pub async fn create(pool: &PgPool, dto: &CreateStageRequestDto) -> AppResult<Stage> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar criação de imagem do projeto".to_string())
        })?;

        let order =
            ordering::next_order(&mut transaction, OrderingTable::Stages, dto.project_id).await?;

        let stage = sqlx::query_as::<_, Stage>(
            r#"
            INSERT INTO stages (
                id,
                project_id,
                name,
                description,
                weight,
                "order",
                status,
                start_date,
                end_date,
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(dto.project_id)
        .bind(&dto.name)
        .bind(&dto.description)
        .bind(dto.weight)
        .bind(order)
        .bind(dto.status)
        .bind(dto.start_date)
        .bind(dto.end_date)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao criar estágio nos dados.".to_string()))?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao concluir criação do estágio do projeto.".to_string())
        })?;

        Ok(stage)
    }

    pub async fn reorder(
        pool: &PgPool,
        project_id: Uuid,
        dto: &ReorderStagesRequestDto,
    ) -> AppResult<()> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar reordenação das imagens do projeto".to_string())
        })?;

        ordering::reorder(
            &mut transaction,
            OrderingTable::Stages,
            project_id,
            &dto.stages,
        )
        .await?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao concluir reordenação das imagens".to_string())
        })
    }

    pub async fn delete(pool: &PgPool, id: Uuid, project_id: Uuid, order: i16) -> AppResult<()> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar deleção de estágio do projeto.".to_string())
        })?;

        let result = sqlx::query(
            r#"
                DELETE FROM project_images
                WHERE id = $1
                  AND project_id = $2
            "#,
        )
        .bind(id)
        .bind(project_id)
        .execute(&mut *transaction)
        .await
        .map_err(|_| {
            AppError::Database("Falha ao excluir estágio do projeto dos dados.".to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(
                "Estágio do projeto não encontrado.".to_string(),
            ));
        }

        ordering::shift_after_delete(&mut transaction, OrderingTable::Stages, project_id, order)
            .await?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao concluir deleção de estágio do projeto.".to_string())
        })?;

        Ok(())
    }
}

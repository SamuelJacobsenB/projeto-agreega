use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::tasks::{
        dtos::{CreateTaskRequestDto, ReorderTasksRequestDto},
        models::Task,
    },
    infrastructure::db::ordering::{self, OrderingTable},
    response::{AppError, AppResult},
};

pub struct Repository;

impl Repository {
    pub async fn find_by_stage_id(pool: &PgPool, stage_id: Uuid) -> AppResult<Vec<Task>> {
        sqlx::query_as::<_, Task>(
            r#"
                SELECT *
                FROM tasks
                WHERE stage_id = $1
            "#,
        )
        .bind(stage_id)
        .fetch_all(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar tarefas nos dados.".to_string()))
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> AppResult<Option<Task>> {
        sqlx::query_as::<_, Task>(
            r#"
                SELECT *
                FROM tasks
                WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|_| AppError::Database("Falha ao buscar tarefa nos dados.".to_string()))
    }

    pub async fn create(pool: &PgPool, dto: &CreateTaskRequestDto) -> AppResult<Task> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar criação de tarefa do projeto".to_string())
        })?;

        let order =
            ordering::next_order(&mut transaction, OrderingTable::Tasks, dto.stage_id).await?;

        let stage = sqlx::query_as::<_, Task>(
            r#"
            INSERT INTO tasks (
                id,
                stage_id,
                title,
                description,
                weight,
                progress,
                "order",
                status,
                assigned_to,
                due_date,
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(dto.stage_id)
        .bind(&dto.title)
        .bind(&dto.description)
        .bind(dto.weight)
        .bind(dto.progress)
        .bind(order)
        .bind(dto.status)
        .bind(dto.assigned_to)
        .bind(dto.due_date)
        .fetch_one(&mut *transaction)
        .await
        .map_err(|_| AppError::Database("Falha ao criar tarefa nos dados.".to_string()))?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao concluir criação da tarefa do estágio.".to_string())
        })?;

        Ok(stage)
    }

    pub async fn reorder(
        pool: &PgPool,
        stage_id: Uuid,
        dto: &ReorderTasksRequestDto,
    ) -> AppResult<()> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar reordenação das taredas do estágio".to_string())
        })?;

        ordering::reorder(&mut transaction, OrderingTable::Tasks, stage_id, &dto.tasks).await?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao concluir reordenação das tarefas".to_string())
        })
    }

    pub async fn delete(pool: &PgPool, id: Uuid, stage_id: Uuid, order: i16) -> AppResult<()> {
        let mut transaction = pool.begin().await.map_err(|_| {
            AppError::Database("Falha ao iniciar deleção de tarefas do estágio.".to_string())
        })?;

        let result = sqlx::query(
            r#"
                DELETE FROM tasks
                WHERE id = $1
                  AND stage_id = $2
            "#,
        )
        .bind(id)
        .bind(stage_id)
        .execute(&mut *transaction)
        .await
        .map_err(|_| {
            AppError::Database("Falha ao excluir tarefa do estágio dos dados.".to_string())
        })?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(
                "Tarefa do estágio não encontrado.".to_string(),
            ));
        }

        ordering::shift_after_delete(&mut transaction, OrderingTable::Tasks, stage_id, order)
            .await?;

        transaction.commit().await.map_err(|_| {
            AppError::Database("Falha ao concluir deleção de tarefa do estágio.".to_string())
        })?;

        Ok(())
    }
}

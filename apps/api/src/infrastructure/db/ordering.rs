use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::response::{AppError, AppResult};

#[derive(Debug, Clone, Copy)]
pub enum OrderingTable {
    ProjectImages,
    Stages,
}

impl OrderingTable {
    fn next_order_query(self) -> &'static str {
        match self {
            Self::ProjectImages => {
                r#"
                    SELECT COALESCE(MAX("order") + 1, 0)
                    FROM project_images
                    WHERE project_id = $1
                "#
            }
            Self::Stages => {
                r#"
                    SELECT COALESCE(MAX("order") + 1, 0)
                    FROM stages
                    WHERE project_id = $1
                "#
            }
        }
    }

    fn shift_after_delete_query(self) -> &'static str {
        match self {
            Self::ProjectImages => {
                r#"
                    UPDATE project_images
                    SET "order" = "order" - 1
                    WHERE project_id = $1
                      AND "order" > $2
                "#
            }
            Self::Stages => {
                r#"
                    UPDATE stages
                    SET "order" = "order" - 1
                    WHERE project_id = $1
                      AND "order" > $2
                "#
            }
        }
    }

    fn prepare_reorder_query(self) -> &'static str {
        match self {
            Self::ProjectImages => {
                r#"
                    UPDATE project_images
                    SET "order" = -"order" - 1
                    WHERE project_id = $1
                "#
            }
            Self::Stages => {
                r#"
                    UPDATE stages
                    SET "order" = -"order" - 1
                    WHERE project_id = $1
                "#
            }
        }
    }

    fn reorder_query(self) -> &'static str {
        match self {
            Self::ProjectImages => {
                r#"
                    UPDATE project_images
                    SET "order" = $1
                    WHERE id = $2
                      AND project_id = $3
                "#
            }
            Self::Stages => {
                r#"
                    UPDATE stages
                    SET "order" = $1
                    WHERE id = $2
                      AND project_id = $3
                "#
            }
        }
    }
}

pub async fn next_order(
    tx: &mut Transaction<'_, Postgres>,
    ordering: OrderingTable,
    parent_id: Uuid,
) -> AppResult<i16> {
    sqlx::query_scalar::<_, i16>(ordering.next_order_query())
        .bind(parent_id)
        .fetch_one(&mut **tx)
        .await
        .map_err(|_| {
            AppError::Database("Falha ao obter a próxima posição de ordenação.".to_string())
        })
}

pub async fn shift_after_delete(
    tx: &mut Transaction<'_, Postgres>,
    ordering: OrderingTable,
    parent_id: Uuid,
    deleted_order: i16,
) -> AppResult<()> {
    sqlx::query(ordering.shift_after_delete_query())
        .bind(parent_id)
        .bind(deleted_order)
        .execute(&mut **tx)
        .await
        .map_err(|_| {
            AppError::Database("Falha ao atualizar a ordenação dos registros.".to_string())
        })?;

    Ok(())
}

pub async fn reorder(
    tx: &mut Transaction<'_, Postgres>,
    ordering: OrderingTable,
    parent_id: Uuid,
    ids: &[Uuid],
) -> AppResult<()> {
    sqlx::query(ordering.prepare_reorder_query())
        .bind(parent_id)
        .execute(&mut **tx)
        .await
        .map_err(|_| {
            AppError::Database("Falha ao preparar a reordenação dos registros.".to_string())
        })?;

    let query = ordering.reorder_query();

    for (order, id) in ids.iter().enumerate() {
        let order = i16::try_from(order).map_err(|_| {
            AppError::BadRequest("Quantidade de registros excede o limite permitido.".to_string())
        })?;

        sqlx::query(query)
            .bind(order)
            .bind(id)
            .bind(parent_id)
            .execute(&mut **tx)
            .await
            .map_err(|_| {
                AppError::Database("Falha ao aplicar a nova ordenação dos registros.".to_string())
            })?;
    }

    Ok(())
}

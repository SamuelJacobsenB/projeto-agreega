use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domains::{projects::service::Service as ProjectService, users::models::UserRole},
    response::{AppError, AppResult},
};

pub async fn authorize_project_access(
    pool: &PgPool,
    project_id: Uuid,
    user_id: Uuid,
    role: UserRole,
) -> AppResult<()> {
    let project = ProjectService::get_project_by_id(pool, project_id).await?;

    if role == UserRole::Client {
        let client_id =
            sqlx::query_scalar::<_, Option<Uuid>>("SELECT client_id FROM users WHERE id = $1")
                .bind(user_id)
                .fetch_optional(pool)
                .await
                .map_err(|_| AppError::Database("Falha ao validar acesso ao projeto.".to_string()))?
                .flatten();

        if client_id != Some(project.client_id) {
            return Err(AppError::Forbidden(
                "Você não tem permissão para acessar este projeto.".to_string(),
            ));
        }
    }

    Ok(())
}

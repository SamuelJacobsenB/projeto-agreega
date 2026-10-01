use sqlx::PgPool;

use crate::{config::Config, infrastructure::email::EmailService};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
    pub email_service: EmailService,
}

use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn create_pool(db_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
}

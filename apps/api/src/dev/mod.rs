use sqlx::PgPool;
use uuid::Uuid;

use crate::{domains::users::models::UserRole, security::password::PasswordService};

pub async fn create_admin_user(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let password_hash = PasswordService::hash_password("Password#123")
        .map_err(|_| std::io::Error::other("Failed to hash admin password"))?;

    sqlx::query(
        r#"
        INSERT INTO users (
            id,
            name,
            email,
            phone,
            password_hash,
            role
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (email) DO NOTHING
        "#,
    )
    .bind(Uuid::new_v4())
    .bind("Admin user")
    .bind("admin@gmail.com")
    .bind("27999999999")
    .bind(password_hash)
    .bind(UserRole::Admin)
    .execute(pool)
    .await?;

    Ok(())
}

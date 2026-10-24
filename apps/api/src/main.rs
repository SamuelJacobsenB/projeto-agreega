use std::net::SocketAddr;

use agreega_api::{
    app::{AppState, router::create_router},
    config::Config,
    infrastructure::{db::connection::create_pool, email::EmailService},
};
use tracing::Level;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let (log_writer, _log_guard) = tracing_appender::non_blocking(std::io::stderr());
    tracing_subscriber::fmt()
        .with_writer(log_writer)
        .with_ansi(false)
        .with_max_level(Level::INFO)
        .try_init()
        .map_err(|error| std::io::Error::other(error.to_string()))?;

    let config = Config::from_env()?;
    let port = config.port;

    let pool = create_pool(&config.database_url).await?;

    let email_service = EmailService::new(config.email_api_key.clone(), config.email_from.clone());

    let state = AppState {
        pool,
        config,
        email_service,
    };

    let app = create_router(state);

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address).await?;

    tracing::info!(port, "Agreega API initialized");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

use std::net::SocketAddr;

use agreega_api::{
    app::{AppState, router::create_router},
    config::Config,
    infrastructure::db::connection::create_pool,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = Config::from_env()?;
    let port = config.port;

    let pool = create_pool(&config.database_url).await?;

    let state = AppState { pool, config };

    let app = create_router(state);

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("... Agreega API initialized ...");

    axum::serve(listener, app).await?;

    Ok(())
}

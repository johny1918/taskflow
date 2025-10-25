mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod routes;
mod services;

use routes::*;
use crate::errors::AppError;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    tracing::info!("Starting TaskFlow backend...");

    // Start Server
    let _ = start_server().await.map_err(|_|AppError::NotFound("Fail to start server".to_string()));

    Ok(())
}

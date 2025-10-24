mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod routes;
mod services;

use routes::*;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    tracing::info!("Starting TaskFlow backend...");

    // Start Server
    start_server().await.expect("Failed to start server");

    Ok(())
}



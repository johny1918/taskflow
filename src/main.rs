mod routes;
mod handlers;
mod models;
mod services;
mod db;
mod errors;
mod config;

use axum::{routing::get, Json, Router};
use serde_json::json;

use crate::config::read_config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    tracing::info!("Starting TaskFlow backend...");

    //Read Config at the moment just PORT
    let config = read_config();

    let app = Router::new()
        .route("/health", get(check_health));
    println!("Listening on {}:{}", config.get_ip(), config.get_port());
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.get_ip(), config.get_port())).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn check_health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}
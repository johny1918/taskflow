mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod routes;
mod services;

use axum::{Json, Router, routing::get};
use serde_json::json;

use crate::config::read_config;
use crate::db::connect_db;
use crate::db::get_tasks;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    tracing::info!("Starting TaskFlow backend...");

    // Read Config
    let config = read_config();
    // Connect to database
    let db = connect_db().await;

    // ✅ CORECT: Folosește for loop sau collect()
    let tasks = get_tasks(&db).await.expect("Failed to get tasks");

    tracing::info!("Found {} tasks:", tasks.len());
    for task in &tasks {
        tracing::info!("Task: {:?}", task);
    }

    // Sau varianta mai scurtă:
    // tasks.iter().for_each(|task| tracing::info!("Task: {:?}", task));

    let app = Router::new().route("/health", get(check_health));
    println!("Listening on {}:{}", config.get_ip(), config.get_port());
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.get_ip(), config.get_port())).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn check_health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

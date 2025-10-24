

use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::get;
use axum::extract::State;
use serde_json::json;
use sqlx::PgPool;
use crate::config::read_config;
use crate::db::fetch_tasks;

pub async fn start_server() -> std::io::Result<()>{
    // Read Config
    let config = read_config();

    // Connect to database
    let pool = crate::db::connect_db().await;

    println!("Listening on {}:{}", config.get_ip(), config.get_port());
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.get_ip(), config.get_port())).await?;
    
    // Start server with database pool in state
    axum::serve(listener, server_paths(pool).await).await?;
    
    Ok(())
}


async fn server_paths(pool: PgPool) -> Router{
    let app: Router = Router::new()
        .route("/health", get(check_health))
        .route("/tasks", get(get_tasks))
        .with_state(pool);

    app
}

async fn get_tasks(State(pool): State<PgPool>) -> Result<Json<serde_json::Value>, StatusCode> {
    match fetch_tasks(&pool).await {
        Ok(tasks) => Ok(Json(json!({ "tasks": tasks }))),
        Err(e) => {
            tracing::error!("Failed to fetch tasks: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn check_health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}
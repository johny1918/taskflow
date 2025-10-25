use crate::config::read_config;
use crate::db::{fetch_tasks, insert_task};
use crate::models::NewTask;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get};
use axum::{Json, Router};
use serde_json::json;
use sqlx::PgPool;

pub async fn start_server() -> std::io::Result<()> {
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

async fn server_paths(pool: PgPool) -> Router {
    let app: Router = Router::new()
        .route("/health", get(check_health))
        .route("/tasks", get(get_tasks).post(create_task))
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

async fn create_task(
    State(pool): State<PgPool>,
    Json(data): Json<NewTask>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Validate title is not empty
    if data.title.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    match insert_task(&pool, data).await {
        Ok(task) => Ok(Json(json!({
            "status": "success",
            "task": task
        }))),
        Err(e) => {
            tracing::error!("Failed to insert task: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn check_health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

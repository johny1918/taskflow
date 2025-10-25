use crate::config::read_config;
use crate::db::{delete, insert, read, update};
use crate::models::NewTask;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
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
        .route("/tasks", get(read_tasks).post(create_task))
        .route("/tasks/{id}", get(get_single_task).put(update_task).delete(delete_task))
        .with_state(pool);

    app
}

async fn read_tasks(State(pool): State<PgPool>) -> Result<Json<serde_json::Value>, StatusCode> {
    match read(&pool).await {
        Ok(tasks) => Ok(Json(json!({ "tasks": tasks }))),
        Err(e) => {
            tracing::error!("Failed to fetch tasks: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_single_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match crate::db::read_one(&pool, id).await {
        Ok(task) => Ok(Json(json!({ "task": task }))),
        Err(e) => {
            tracing::error!("Failed to fetch task: {:?}", e);
            Err(StatusCode::NOT_FOUND)
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
    match insert(&pool, data).await {
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

async fn delete_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    match delete(&pool, id).await {
        Ok(_) => Ok(Json(json!({ "status": "Task deleted" }))),
        Err(e) => {
            tracing::error!("Failed to delete task: {:?}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn update_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(data): Json<NewTask>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Validate title is not empty
    if data.title.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    match update(&pool, id, data).await {
        Ok(task) => Ok(Json(json!({
            "status": "success",
            "task updated with success": task
        }))),
        Err(e) => {
            tracing::error!("Failed to update task: {:?}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn check_health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

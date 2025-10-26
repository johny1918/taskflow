use crate::config::read_config;
use crate::db::{delete, insert, read, read_one, update};
use crate::errors::AppError;
use crate::errors::AppError::DatabaseError;
use crate::models::NewTask;
use crate::models::TaskFilter;
use axum::extract::Query;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use sqlx::PgPool;

pub async fn start_server() -> Result<(), AppError> {
    // Read Config
    let config = read_config();

    // Connect to database
    let pool = crate::db::connect_db().await?;

    tracing::info!("Listening on {}:{}", config.get_ip(), config.get_port());
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.get_ip(), config.get_port()))
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to bind to address: {}", e)))?;

    // Start server with database pool in state
    axum::serve(listener, server_paths(pool).await)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Server error: {}", e)))?;

    Ok(())
}

async fn server_paths(pool: PgPool) -> Router {
    let app: Router = Router::new()
        .route("/health", get(check_health))
        .route("/tasks", get(read_tasks).post(create_task))
        .route(
            "/tasks/{id}",
            get(get_single_task).put(update_task).delete(delete_task),
        )
        .with_state(pool);

    app
}

async fn read_tasks(
    State(pool): State<PgPool>,
    Query(par): Query<TaskFilter>,
) -> Result<Json<serde_json::Value>, AppError> {
    let tasks = read(&pool, par.done).await.map_err(|e| {
        e.to_string();
    });
    Ok(Json(json!({ "tasks": tasks })))
}

async fn get_single_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, AppError> {
    let task = read_one(&pool, id)
        .await
        .map_err(|e| DatabaseError(e.to_string()));
    tracing::info!("Task with id {} has been read with success", id);
    Ok(Json(json!({ "task": task })))
}

async fn create_task(
    State(pool): State<PgPool>,
    Json(data): Json<NewTask>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Validate title is not empty
    if data.title.trim().is_empty() {
        tracing::error!("Fail to create task: title is empty");
        return Err(AppError::NotFound("Title is empty".to_string()));
    }
    let task = insert(&pool, data)
        .await
        .map_err(|e| DatabaseError(e.to_string()));

    tracing::info!("Task created with success");
    Ok(Json(json!({
        "status": "success",
        "task": task
    })))
}

async fn delete_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, AppError> {
    let task = delete(&pool, id)
        .await
        .map_err(|e| DatabaseError(e.to_string()));
    if task? == 0u64 {
        return Err(AppError::NotFound("Task not found".into()));
    }

    tracing::info!("Task deleted with success");
    Ok(Json(json!({ "status": "success", "deleted": id })))
}

async fn update_task(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(data): Json<NewTask>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Validate title is not empty
    if data.title.trim().is_empty() {
        tracing::error!("Fail to update task: title is empty");
        return Err(AppError::NotFound("Title is empty".to_string()));
    }

    let task = update(&pool, id, data)
        .await
        .map_err(|e| DatabaseError(e.to_string()));

    if task.is_err() {
        return Err(AppError::NotFound(format!("Task with id {} not found", id)));
    }

    tracing::info!("Task updated with success");
    Ok(Json(json!({ "status": "success", "updated": id })))
}

async fn check_health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

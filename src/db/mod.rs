use crate::config::read_config;
use crate::errors::AppError;
use crate::models::*;
use sqlx::PgPool;

pub async fn connect_db() -> Result<PgPool, AppError> {
    // Read config for .env variables
    let config = read_config();
    let database_url = config.get_database_url();

    //Create a connection pool
    let pool = PgPool::connect(&database_url)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Fail to connect to database: {}", e)))?;
    tracing::info!("Connected to database");

    Ok(pool)
}

pub async fn read(db: &PgPool) -> Result<Vec<Task>, AppError> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(db)
        .await
        .map_err(|_| AppError::DatabaseError("Fail to read all tasks from database".to_string()))?;
    tracing::info!("Read all query executed successfully");
    Ok(task)
}

pub async fn read_one(db: &PgPool, id: i32) -> Result<Task, AppError> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
        .bind(id)
        .fetch_one(db)
        .await
        .map_err(|_| {
            AppError::DatabaseError("Fail to read just one task from database".to_string())
        })?;
    tracing::info!("Read just one task query executed successfully");
    Ok(task)
}

pub async fn insert(db: &PgPool, task: NewTask) -> Result<Task, AppError> {
    let result =
        sqlx::query_as::<_, Task>(r#"INSERT INTO tasks (title, done) VALUES ($1, $2) RETURNING *"#)
            .bind(&task.title)
            .bind(task.done)
            .fetch_one(db)
            .await
            .map_err(|_| {
                AppError::DatabaseError("Fail to insert task into database".to_string())
            })?;
    tracing::info!("Insert task query executed successfully");
    Ok(result)
}

pub async fn delete(db: &PgPool, id: i32) -> Result<(), AppError> {
    sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(id)
        .execute(db)
        .await
        .map_err(|_| AppError::DatabaseError("Fail to delete task from database".to_string()))?;
    tracing::info!("Delete just one task query executed successfully");
    Ok(())
}

pub async fn update(db: &PgPool, id: i32, task: NewTask) -> Result<Task, AppError> {
    let result = sqlx::query_as::<_, Task>(
        r#"UPDATE tasks SET title = $1, done = $2 WHERE id = $3 RETURNING *"#,
    )
    .bind(&task.title)
    .bind(task.done)
    .bind(id)
    .fetch_one(db)
    .await
    .map_err(|_| AppError::DatabaseError("Fail to update task from database".to_string()))?;
    tracing::info!("Update task query executed successfully");
    Ok(result)
}

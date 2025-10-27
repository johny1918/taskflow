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

pub async fn read(
    db: &PgPool,
    done_filter: Option<bool>,
    limit: i64,
    offset: i64,
) -> Result<Vec<Task>, AppError> {
    let mut query = format!("SELECT * FROM tasks LIMIT {limit} OFFSET {offset}");
    if let Some(done_filter) = done_filter {
        if done_filter {
            query = format!(
                "SELECT * FROM tasks WHERE done = {done_filter} LIMIT {limit} OFFSET {offset}"
            );
            tracing::info!("Read query by filter executed successfully");
        } else if !done_filter {
            query = format!(
                "SELECT * FROM tasks WHERE done = {done_filter} LIMIT {limit} OFFSET {offset}"
            );
            tracing::info!("Read query by filter executed successfully");
        }
    }

    let task = sqlx::query_as::<_, Task>(query.as_str())
        .fetch_all(db)
        .await
        .map_err(|_| AppError::DatabaseError("Fail to read all tasks from database".to_string()))?;
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

pub async fn delete(db: &PgPool, id: i32) -> Result<u64, AppError> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(id)
        .execute(db)
        .await
        .map_err(|_| AppError::DatabaseError("Fail to delete task from database".to_string()))?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Task not found".into()));
    }
    tracing::info!("Delete task query executed successfully");
    Ok(result.rows_affected())
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
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => AppError::NotFound("Task not found".into()),
        _ => AppError::DatabaseError("Fail to update task from database".to_string()),
    });

    tracing::info!("Update task query executed successfully");
    result
}

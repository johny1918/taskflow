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
    sort: Option<String>,
    order: Option<String>,
) -> Result<Vec<Task>, AppError> {
    let mut query = String::from("SELECT * FROM tasks");

    // Filter by done if specified
    if let Some(done) = done_filter {
        query.push_str(&format!(" WHERE done = {}", done));
    }

    // Sorting
    if let Some(sort_by) = sort {
        let column = match sort_by.as_str() {
            "id" | "title" | "done" => sort_by,
            _ => "id".to_string(), // default safe fallback
        };

        let direction = match order.clone().unwrap_or_else(|| "asc".to_string()).as_str() {
            "asc" | "desc" => order.unwrap(),
            _ => "asc".to_string(), // default safe fallback
        };

        query.push_str(&format!(" ORDER BY {} {}", column, direction));
    }

    query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    let tasks = sqlx::query_as::<_, Task>(&query)
        .bind(limit)
        .bind(offset)
        .fetch_all(db)
        .await
        .map_err(|_| AppError::DatabaseError("Fail to read tasks".to_string()))?;

    Ok(tasks)
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

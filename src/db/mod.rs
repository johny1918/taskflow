use crate::config::read_config;
use crate::models::*;
use sqlx::PgPool;

pub async fn connect_db() -> PgPool {
    // Read config for .env variables
    let config = read_config();
    let database_url = config.get_database_url();

    //Create a connection pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    tracing::info!("Connected to database");

    pool
}

pub async fn read(db: &PgPool) -> Result<Vec<Task>, sqlx::Error> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(db)
        .await?;
    Ok(task)
}

pub async fn read_one(db: &PgPool, id: i32) -> Result<Task, sqlx::Error> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = $1")
        .bind(id)
        .fetch_one(db)
        .await?;
    Ok(task)
}

pub async fn insert(db: &PgPool, task: NewTask) -> Result<Task, sqlx::Error> {
    let result = sqlx::query_as::<_, Task>(
        r#"INSERT INTO tasks (title, done) VALUES ($1, $2) RETURNING *"#,
    )
        .bind(&task.title)
        .bind(task.done)
        .fetch_one(db)
        .await?;
    Ok(result)
}

pub async fn delete(db: &PgPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn update(db: &PgPool, id: i32, task: NewTask) -> Result<Task, sqlx::Error> {
    let result = sqlx::query_as::<_, Task>(
        r#"UPDATE tasks SET title = $1, done = $2 WHERE id = $3 RETURNING *"#,
    )
        .bind(&task.title)
        .bind(task.done)
        .bind(id)
        .fetch_one(db)
        .await?;
    Ok(result)
}

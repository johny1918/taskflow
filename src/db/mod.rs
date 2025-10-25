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

pub async fn fetch_tasks(db: &PgPool) -> Result<Vec<Task>, sqlx::Error> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(db)
        .await?;
    Ok(task)
}

pub async fn insert_task(db: &PgPool, task: NewTask) -> Result<Vec<NewTask>, sqlx::Error> {
    let result = sqlx::query_as::<_, NewTask> (
        r#"INSERT INTO tasks (title, done) VALUES ($1, $2) RETURNING *"#,
    ).bind(&task.title)
        .bind(task.done)
        .fetch_all(db)
        .await?;
    Ok(result)
}

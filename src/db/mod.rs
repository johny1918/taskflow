use crate::config::read_config;
use sqlx::PgPool;
use tracing;
use crate::models::Task;

pub async fn connect_db() -> PgPool {
    // Read config for .env variables
    let config = read_config();
    let database_url = config.get_database_url();

    //Create connection pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    tracing::info!("Connected to database");

    pool
}

pub async fn get_tasks(db: &PgPool) -> Result<Vec<Task>, sqlx::Error> {
    let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(db)
        .await?;
    Ok(task)
}

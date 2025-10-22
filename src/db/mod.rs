use sqlx::PgPool;
use tracing;
use crate::config::read_config;
pub async fn connect_db() ->std::io::Result<()> {
    // Read config for .env variables
    let config = read_config();
    let database_url = config.get_database_url();

    //Create connection pool
    let pool = PgPool::connect(&database_url).await;
    tracing::info!("Connected to database");

    Ok(())
}
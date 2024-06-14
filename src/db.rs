use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: MySqlPool,
}

pub async fn establish_connection() -> MySqlPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create pool.")
}

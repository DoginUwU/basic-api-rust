use std::env;

use dotenv::dotenv;
use sqlx::{SqlitePool};

pub async fn get_pool() -> SqlitePool  {
    dotenv().ok();
    sqlx::migrate!();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    SqlitePool::connect(&database_url).await.expect("Failed to connect to DB")
}
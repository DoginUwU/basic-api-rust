use std::env;

use dotenv::dotenv;
use sqlx::{SqlitePool};
use lazy_static::lazy_static;
use async_once::AsyncOnce;

lazy_static! {
    static ref DB_POOL: AsyncOnce<SqlitePool> = {
        sqlx::migrate!();

        AsyncOnce::new(async {
            dotenv().ok();

            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

            SqlitePool::connect(&database_url).await.expect("Failed to connect to DB")
        })
    };
}

pub async fn get_pool() -> &'static SqlitePool {
    DB_POOL.get().await
}
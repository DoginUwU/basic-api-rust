mod routes;
mod controllers;
mod models;
mod db;

use routes::user_routes;
use axum::{ routing::get, Router };

use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: SqlitePool,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        db_pool: db::get_pool().await,
    };

    let app = Router::new()
    .route("/", get(|| async { "OK" }))
    .nest("/users", user_routes::routes())
    .with_state(state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
    .unwrap();
}

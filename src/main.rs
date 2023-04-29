mod routes;
mod controllers;
mod models;
mod db;

use axum::{ routing::get, Router };

use routes::user_routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(|| async { "OK" }))
    .nest("/users", user_routes::routes());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
    .unwrap();
}

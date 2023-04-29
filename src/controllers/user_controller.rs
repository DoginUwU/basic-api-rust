use axum::{response::{Json}, http::StatusCode, extract};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::db::get_pool;

pub async fn get_user(params: extract::Path<String>) -> Result<Json<Value>, StatusCode> {
    let user_email = params.0;
    let pool = get_pool().await;

    let user = sqlx::query!("SELECT * FROM users WHERE email = ?", user_email)
        .fetch_one(pool)
        .await
        .ok();

    match user {
        Some(user) => Ok(Json(json!({
            "name": user.name,
            "email": user.email,
        }))),
        None => Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn add_user(Json(payload): Json<CreateUserRequest>) -> Result<Json<Value>, StatusCode> {
    let pool = get_pool().await;

    let user = sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES (?, ?, ?)",
        payload.name,
        payload.email,
        payload.password
    )
    .execute(pool)
    .await
    .ok();

    match user {
        Some(_) => Ok(Json(json!({
            "name": payload.name,
            "email": payload.email,
        }))),
        None => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
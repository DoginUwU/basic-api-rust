use axum::{response::Json, http::StatusCode, extract::{self, State}};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::AppState;

pub async fn get_user(State(state): State<AppState>, params: extract::Path<String>) -> Result<Json<Value>, StatusCode> {
    let user_id = params.0;

    let user = sqlx::query!("SELECT * FROM users WHERE id = ?", user_id)
        .fetch_one(&state.db_pool)
        .await
        .ok();

    match user {
        Some(user) => Ok(Json(json!({
            "id": user.id,
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

pub async fn add_user(State(state): State<AppState>, Json(payload): Json<CreateUserRequest>) -> Result<Json<Value>, StatusCode> {
    let id = uuid::Uuid::new_v4().to_string();

    let user = sqlx::query!(
        "INSERT INTO users (id, name, email, password) VALUES (?, ?, ?, ?)",
        id,
        payload.name,
        payload.email,
        payload.password
    )
    .execute(&state.db_pool)
    .await
    .ok();

    match user {
        Some(_) => Ok(Json(json!({
            "id": id,
            "name": payload.name,
            "email": payload.email,
        }))),
        None => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
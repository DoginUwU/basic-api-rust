use axum::{ routing::get, routing::post, Router };

use crate::controllers::user_controller;

pub fn routes() -> Router {
    Router::new()
        .route("/:email", get(user_controller::get_user))
        .route("/", post(user_controller::add_user))
}
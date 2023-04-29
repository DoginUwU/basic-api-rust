use axum::{ routing::{get, post}, Router };

use crate::{controllers::user_controller, AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/:id", get(user_controller::get_user))
        .route("/", post(user_controller::add_user))
}
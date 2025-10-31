use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{current_user, login, register},
    state::AppState,
};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(register))
        .route("/users/login", post(login))
        .route("/user", get(current_user))
}

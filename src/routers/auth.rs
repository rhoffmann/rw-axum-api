use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    handlers::{forgot_password, logout, refresh_token, reset_password, verify_email},
    state::AppState,
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/verify-email", get(verify_email))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
        .route("/refresh", post(refresh_token))
        .route("/logout", post(logout))
}

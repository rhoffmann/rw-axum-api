mod auth;
mod handlers;
mod models;
mod repositories;
mod schemas;
mod services;
mod state;
mod utils;

use std::env;

use axum::{Router, routing::get, routing::post};

use rw_axum_api::{
    handlers::{current_user, health_check, login, register},
    state::AppState,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database = env::var("DATABASE_URL").expect("DATABASE_URL must be defined");
    let bind_addr = env::var("BIND_ADDR").unwrap_or("0.0.0.0:3000".to_string());

    let app_state = AppState::new(&database)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database successfully!");

    let app = Router::new()
        // health check
        .route("/health", get(health_check))
        // auth
        .route("/api/users", post(register))
        .route("/api/users/login", post(login))
        .route("/api/user", get(current_user))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();

    println!("Server is running on {}", &bind_addr);
    println!("Available endpoints:");
    println!("  POST /api/users         - Register new user");
    println!("  POST /api/users/login   - Login existing user");
    println!("  GET  /api/user          - Get current user (requires auth)");
    println!("  GET  /health            - Health check");

    axum::serve(listener, app).await.unwrap();
}

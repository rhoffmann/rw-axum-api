mod handlers;
mod state;

use std::env;

use axum::{Router, routing::get};

use crate::{handlers::health::health_check, state::AppState};

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
        .with_state(app_state)
        .route("/health", get(health_check));

    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();

    println!("Server is running on {}", &bind_addr);

    axum::serve(listener, app).await.unwrap();
}

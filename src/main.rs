use axum::{Json, Router, routing::get};
use serde_json::{Value, json};
#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:4444";
    let app = Router::new().route("/health", get(health_check));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server is running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "Server is running"
    }))
}

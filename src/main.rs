use std::env;

use axum::{Router, routing::get};

use rw_axum_api::{
    handlers::{health_check, root_handler},
    routers::{auth_routes, create_static_asset_router, user_routes},
    state::AppState,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber for logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let database = env::var("DATABASE_URL").expect("DATABASE_URL must be defined");
    let bind_addr = env::var("BIND_ADDR").unwrap_or("0.0.0.0:3000".to_string());

    let app_state = AppState::new(&database)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database successfully!");

    let app = Router::new()
        .route("/", get(root_handler))
        // health check
        .route("/health", get(health_check))
        // api
        .nest(
            "/api",
            Router::new()
                .merge(user_routes())
                .nest("/auth", auth_routes()),
        )
        // serve static assets
        .merge(create_static_asset_router(&app_state.static_asset_dir))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();

    println!("Server is running on {}", &bind_addr);
    println!("Available endpoints:");
    println!("  POST /api/users                     - Register new user");
    println!("  POST /api/users/login               - Login existing user");
    println!("  GET  /api/user                      - Get current user (requires auth)");
    println!("  GET  /api/auth/verify-email         - Verify email with token");
    println!("  POST /api/auth/forgot-password      - Request new password");
    println!("  POST /api/auth/reset-password       - Validate password reset token");
    println!("  POST /api/auth/refresh              - Refresh Access-Token");
    println!("  POST /api/auth/logout               - Logout (delete refresh token)");
    println!("  GET  /health                        - Health check");

    axum::serve(listener, app).await.unwrap();
}

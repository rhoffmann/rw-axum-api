use axum::{
    Router,
    extract::State,
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
};
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::state::AppState;

async fn spa_fallback(State(state): State<AppState>, uri: Uri) -> Response {
    let index_path = format!("{}/index.html", state.static_asset_dir);
    println!(
        "SPA fallback triggered for URI: {} (serving {})",
        uri.path(),
        index_path
    );
    match tokio::fs::read(&index_path).await {
        Ok(contents) => (StatusCode::OK, [("content-type", "text/html")], contents).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub fn create_static_asset_router(dir: &str) -> Router<AppState> {
    let asset_router = Router::new()
        .nest_service(
            "/web",
            ServeDir::new(dir).append_index_html_on_directories(true),
        )
        .fallback(spa_fallback)
        .layer(TraceLayer::new_for_http());

    println!("Serving static assets from: {}", &dir);

    asset_router
}

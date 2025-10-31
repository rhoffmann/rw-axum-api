use axum::{
    extract::State,
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
};

use crate::state::AppState;

pub async fn spa_fallback(State(state): State<AppState>, uri: Uri) -> Response {
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

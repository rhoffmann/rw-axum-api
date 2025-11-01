use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::state::AppState;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    app_name: String,
}

pub async fn root_handler(State(state): State<AppState>) -> impl IntoResponse {
    let template = IndexTemplate {
        app_name: state.app_name.clone(),
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template: {}", e),
        )
            .into_response(),
    }
}

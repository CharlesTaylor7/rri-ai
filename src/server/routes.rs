use super::state::AppState;
use crate::templates::*;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use std::borrow::Cow;
use tower_http::services::ServeDir;

pub fn get_router() -> Router {
    let context = AppState::default();

    Router::new()
        .route("/", get(index))
        .route("/version", get(get_version))
        .nest_service("/public", ServeDir::new("public"))
        .nest_service("/styles", ServeDir::new("styles"))
        .with_state(context)
}

pub async fn index() -> impl IntoResponse {
    GameTemplate {}.to_html()
}

pub async fn get_version() -> impl IntoResponse {
    std::env::var("VERSION")
        .map_or(Cow::Borrowed("dev"), Cow::Owned)
        .into_response()
}

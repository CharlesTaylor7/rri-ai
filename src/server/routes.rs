use super::state::AppState;
use crate::templates::*;
use axum::extract::{Form, Path, State};
use axum::response::{ErrorResponse, Html, Redirect, Response, Result};
use axum::routing::{get, post};
use axum::Router;
use axum::{extract::ws::WebSocketUpgrade, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};
use http::{Request, StatusCode};
use std::borrow::{Borrow, Cow};
use std::collections::{HashMap, HashSet};
use tower::{ServiceBuilder, ServiceExt};
use tower_http::services::fs;
use tower_http::services::ServeDir;
use uuid::Uuid;

async fn not_found() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}

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

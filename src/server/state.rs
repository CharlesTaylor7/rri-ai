use axum::extract::FromRef;
use axum_extra::extract::cookie;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    cookie_signing_key: cookie::Key,
}

impl Default for AppState {
    fn default() -> Self {
        let key = std::env::var("COOKIE_SIGNING_KEY").expect("env var COOKIE_SIGNING_KEY not set");
        Self {
            cookie_signing_key: cookie::Key::from(key.as_bytes()),
        }
    }
}

impl FromRef<AppState> for cookie::Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_signing_key.clone()
    }
}

use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::api::SESSION_EXPIRED;
use crate::types::User;

pub const AUTH_KEY: &str = "blog_auth";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_name = atob)]
    fn js_atob(s: &str) -> Result<String, JsValue>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthState {
    pub token: String,
    pub user: User,
}

/// Decode JWT payload and check if `exp` is in the past.
/// Returns `true` for any malformed token (fail-safe).
pub fn is_token_expired(token: &str) -> bool {
    let Some(payload) = token.split('.').nth(1) else {
        return true;
    };
    let mut b64 = payload.replace('-', "+").replace('_', "/");
    match b64.len() % 4 {
        2 => b64.push_str("=="),
        3 => b64.push('='),
        _ => {}
    }
    let Ok(decoded) = js_atob(&b64) else {
        return true;
    };
    let Ok(json) = serde_json::from_str::<serde_json::Value>(&decoded) else {
        return true;
    };
    let Some(exp) = json.get("exp").and_then(|v| v.as_f64()) else {
        return true;
    };
    let now = js_sys::Date::now() / 1000.0;
    now >= exp
}

/// Discards expired tokens from localStorage on startup.
pub fn provide_auth_context() {
    let initial: Option<AuthState> = LocalStorage::get(AUTH_KEY)
        .ok()
        .filter(|a: &AuthState| !is_token_expired(&a.token));

    if initial.is_none() {
        let _ = LocalStorage::delete(AUTH_KEY);
    }

    let auth = RwSignal::new(initial);
    provide_context(auth);
}

/// Must be called within a reactive owner.
pub fn use_auth() -> RwSignal<Option<AuthState>> {
    expect_context::<RwSignal<Option<AuthState>>>()
}

/// Clears both localStorage and the reactive signal.
pub fn force_logout(auth: RwSignal<Option<AuthState>>) {
    let _ = LocalStorage::delete(AUTH_KEY);
    auth.set(None);
}

/// If `error` indicates an expired/invalid session, clears the auth signal
/// so the UI reacts immediately (hides owner buttons, triggers auth-guard redirects).
/// Returns `true` if the session was cleared.
pub fn clear_if_unauthorized(error: &str, auth: RwSignal<Option<AuthState>>) -> bool {
    if error == SESSION_EXPIRED {
        force_logout(auth);
        true
    } else {
        false
    }
}

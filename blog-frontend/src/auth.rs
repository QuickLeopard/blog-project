use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types::User;

const AUTH_KEY: &str = "blog_auth";

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthState {
    pub token: String,
    pub user: User,
}

// Call once at app startup inside App component:
pub fn provide_auth_context() {
    // Try to load previously saved auth from localStorage
    let initial: Option<AuthState> = LocalStorage::get(AUTH_KEY).ok();

    // Create a reactive signal with the loaded value (or None if no saved session)
    let auth = RwSignal::new(initial);

    // Make it available to all child components via Leptos context
    provide_context(auth);
}

// Use inside any component (must be called within reactive owner):
pub fn use_auth() -> RwSignal<Option<AuthState>> {
    expect_context::<RwSignal<Option<AuthState>>>()
}

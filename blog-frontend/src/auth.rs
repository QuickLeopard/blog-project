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

// Use inside any component:
pub fn use_auth() -> RwSignal<Option<AuthState>> {
    // Read the signal from context (panics if called outside of App — which is correct)
    expect_context::<RwSignal<Option<AuthState>>>()
}

// Call after successful login/register:
pub fn set_auth(state: AuthState) {
    // Persist to localStorage
    LocalStorage::set(AUTH_KEY, &state).ok();
    // Update the reactive signal — all subscribed components re-render
    use_auth().set(Some(state));
} // saves to localStorage + updates signal

// Call on logout:
pub fn clear_auth() {
    // Remove from localStorage
    LocalStorage::delete(AUTH_KEY);
    // Clear the signal — navbar, protected pages react immediately
    use_auth().set(None);
} // removes from localStorage + sets signal to None

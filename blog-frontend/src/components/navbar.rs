use leptos::prelude::*;
use leptos_router::components::A;

use crate::auth::{clear_auth, use_auth};

#[component]
pub fn Navbar() -> impl IntoView {
    let auth = use_auth();

    // Reactive derived signals — re-evaluate whenever auth changes
    let is_logged_in = move || auth.get().is_some();
    let username = move || {
        auth.get()
            .map(|a| a.user.username.clone())
            .unwrap_or_default()
    };

    let on_logout = move |_| {
        clear_auth();
    };

    view! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark px-3">
            <A attr:class="navbar-brand" href="/">"Blog"</A>
            <div class="navbar-nav ms-auto">
                {move || {
                    if is_logged_in() {
                        view! {
                            <span class="nav-link text-light">{username()}</span>
                            <A attr:class="nav-link" href="/posts/new">"New Post"</A>
                            <button class="btn btn-outline-light btn-sm" on:click=on_logout>"Logout"</button>
                        }.into_any()
                    } else {
                        view! {
                            <A attr:class="nav-link" href="/login">"Login"</A>
                            <A attr:class="nav-link" href="/register">"Register"</A>
                        }.into_any()
                    }
                }}
            </div>
        </nav>
    }
}

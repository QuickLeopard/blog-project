use leptos::prelude::*;
use leptos_router::components::A;

use crate::auth::{force_logout, use_auth};

#[component]
pub fn Navbar() -> impl IntoView {
    let auth = use_auth();

    let is_logged_in = move || auth.get().is_some();
    let username = move || {
        auth.get()
            .map(|a| a.user.username.clone())
            .unwrap_or_default()
    };

    let on_logout = move |_| {
        force_logout(auth);
    };

    view! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark px-4">
            <A attr:class="navbar-brand fs-5 fw-semibold" href="/">"Blog"</A>
            <div class="d-flex align-items-center ms-auto gap-2">
                {move || {
                    if is_logged_in() {
                        view! {
                            <A attr:class="btn btn-sm btn-outline-light" href="/posts/new">"New Post"</A>
                            <span class="badge bg-secondary fs-6 fw-normal px-3 py-2">{username()}</span>
                            <button class="btn btn-sm btn-danger" on:click=on_logout>"Logout"</button>
                        }.into_any()
                    } else {
                        view! {
                            <A attr:class="btn btn-sm btn-outline-light" href="/login">"Login"</A>
                            <A attr:class="btn btn-sm btn-light" href="/register">"Register"</A>
                        }.into_any()
                    }
                }}
            </div>
        </nav>
    }
}

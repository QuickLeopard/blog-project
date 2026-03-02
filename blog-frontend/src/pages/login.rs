// src/pages/login.rs
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

use crate::api;
use crate::auth::{set_auth, AuthState};

#[component]
pub fn Login() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error    = RwSignal::new(None::<String>);

    let navigate = use_navigate();

    // Action::new_local — takes one closure that owns its inputs
    let login_action = Action::new_local(move |_: &()| {
        let username = username.get();
        let password = password.get();
        let navigate = navigate.clone();

        async move {
            match api::login(&username, &password).await {
                Ok(resp) => {
                    set_auth(AuthState {
                        token: resp.token,
                        user:  resp.user,
                    });
                    error.set(None);
                    navigate("/", Default::default());
                }
                Err(e) => {
                    error.set(Some(e));
                }
            }
        }
    });

    let pending = login_action.pending();

    view! {
        <div class="container mt-5" style="max-width: 400px">
            <h2 class="mb-4">"Login"</h2>

            // Error banner
            {move || error.get().map(|msg| view! {
                <div class="alert alert-danger">{msg}</div>
            })}

            <div class="mb-3">
                <label class="form-label">"Username"</label>
                <input
                    class="form-control"
                    type="text"
                    prop:value=move || username.get()
                    on:input=move |ev| username.set(event_target_value(&ev))
                />
            </div>
            <div class="mb-3">
                <label class="form-label">"Password"</label>
                <input
                    class="form-control"
                    type="password"
                    prop:value=move || password.get()
                    on:input=move |ev| password.set(event_target_value(&ev))
                />
            </div>

            <button
                class="btn btn-primary w-100"
                disabled=move || pending.get()
                on:click=move |_| { login_action.dispatch(()); }
            >
                {move || if pending.get() { "Logging in…" } else { "Login" }}
            </button>

            <p class="mt-3 text-center">
                "No account? "
                <A href="/register">"Register"</A>
            </p>
        </div>
    }
}
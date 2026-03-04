use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

use gloo_storage::Storage;

use crate::api;
use crate::auth::{use_auth, AuthState, AUTH_KEY};

#[component]
pub fn Register() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error    = RwSignal::new(None::<String>);

    let navigate = use_navigate();
    let auth = use_auth();

    let register_action = Action::new_local(move |_: &()| {
        let username = username.get();
        let email = email.get();
        let password = password.get();
        let navigate = navigate.clone();

        async move {
            match api::register(&username, &email, &password).await {
                Ok(resp) => {
                    let state = AuthState {
                        token: resp.token,
                        user:  resp.user,
                    };
                    if let Err(e) = gloo_storage::LocalStorage::set(AUTH_KEY, &state) {
                        error.set(Some(format!("Failed to save session: {e}")));
                        return;
                    }
                    auth.set(Some(state));
                    error.set(None);
                    navigate("/", Default::default());
                }
                Err(e) => {
                    error.set(Some(e));
                }
            }
        }
    });

    let pending = register_action.pending();

    view! {
        <div class="container py-5 d-flex justify-content-center">
            <div style="width: 100%; max-width: 420px">
                <h2 class="page-heading text-center mb-4">"Create an account"</h2>
                <div class="auth-card card p-4">
                    {move || error.get().map(|msg| view! {
                        <div class="alert alert-danger">{msg}</div>
                    })}

                    <div class="mb-3">
                        <label class="form-label">"Username"</label>
                        <input
                            class="form-control"
                            type="text"
                            placeholder="your_username"
                            prop:value=move || username.get()
                            on:input=move |ev| username.set(event_target_value(&ev))
                        />
                    </div>
                    <div class="mb-3">
                        <label class="form-label">"Email"</label>
                        <input
                            class="form-control"
                            type="email"
                            placeholder="you@example.com"
                            prop:value=move || email.get()
                            on:input=move |ev| email.set(event_target_value(&ev))
                        />
                    </div>
                    <div class="mb-4">
                        <label class="form-label">"Password"</label>
                        <input
                            class="form-control"
                            type="password"
                            placeholder="••••••••"
                            prop:value=move || password.get()
                            on:input=move |ev| password.set(event_target_value(&ev))
                        />
                    </div>

                    <button
                        class="btn btn-primary w-100 py-2"
                        disabled=move || pending.get() || username.get().trim().is_empty() || email.get().trim().is_empty() || password.get().trim().is_empty()
                        on:click=move |_| { register_action.dispatch(()); }
                    >
                        {move || if pending.get() {
                            view! {
                                <span>
                                    <span class="spinner-border spinner-border-sm me-2" role="status"></span>
                                    "Creating account…"
                                </span>
                            }.into_any()
                        } else {
                            view! { <span>"Register"</span> }.into_any()
                        }}
                    </button>

                    <p class="mt-3 mb-0 text-center text-muted small">
                        "Already have an account? "
                        <A href="/login">"Log in"</A>
                    </p>
                </div>
            </div>
        </div>
    }
}
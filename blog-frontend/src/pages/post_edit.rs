use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::api;
use crate::auth::use_auth;
use crate::components::post_form::PostForm;

#[component]
pub fn PostEdit() -> impl IntoView {
    let params = use_params_map();
    let auth = use_auth();
    let navigate = use_navigate();
    let error = RwSignal::new(None::<String>);

    let navigate_redirect = navigate.clone();

    // Redirect to /login if not authenticated — mirrors post_create.rs exactly
    Effect::new(move |_| {
        if auth.get().is_none() {
            navigate_redirect("/login", Default::default());
        }
    });

    // Parse :id from the route — mirrors post_detail.rs exactly
    let id = move || {
        params
            .get()
            .get("id")
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(0)
    };

    // Fetch the existing post to pre-fill the form — mirrors post_detail.rs
    let post_resource = LocalResource::new(move || {
        let post_id = id();
        async move { api::get_post(post_id).await }
    });

    // Action that calls api::update_post — mirrors post_create.rs Action pattern
    let update_action = Action::new_local(move |(title, content): &(String, String)| {
        let title = title.clone();
        let content = content.clone();
        let post_id = id();
        let token = auth.get().map(|a| a.token.clone()).unwrap_or_default();
        let navigate = navigate.clone();
        let error = error;

        async move {
            match api::update_post(post_id, &title, &content, &token).await {
                Ok(post) => {
                    error.set(None);
                    navigate(&format!("/posts/{}", post.id), Default::default());
                }
                Err(e) => error.set(Some(e)),
            }
        }
    });

    // Callback<(String, String)> — same shape as post_create.rs
    let on_submit = Callback::new(move |(title, content): (String, String)| {
        update_action.dispatch((title, content));
    });

    view! {
        <div class="container py-4" style="max-width: 720px">
            <div class="d-flex align-items-center gap-3 mb-4">
                <A attr:class="btn btn-sm btn-outline-secondary" href=move || format!("/posts/{}", id())>
                    "← Back"
                </A>
                <h2 class="page-heading mb-0">"Edit Post"</h2>
            </div>
            <Suspense fallback=move || view! { <p class="loading-text">"Loading post…"</p> }>
                {move || {
                    post_resource.get().map(|result| {
                        let result = (*result).clone();
                        match result {
                            Err(err) => view! {
                                <div class="alert alert-danger">{err}</div>
                            }.into_any(),
                            Ok(post) => view! {
                                <PostForm
                                    initial_title=post.title
                                    initial_content=post.content
                                    on_submit=on_submit
                                    loading=update_action.pending().into()
                                    error=error
                                />
                            }.into_any(),
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
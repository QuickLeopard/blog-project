use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::api;
use crate::auth::{clear_if_unauthorized, force_logout, is_token_expired, use_auth};
use crate::components::post_form::PostForm;

#[component]
pub fn PostEdit() -> impl IntoView {
    let params = use_params_map();
    let auth = use_auth();
    let navigate = use_navigate();
    let error = RwSignal::new(None::<String>);

    let navigate_redirect = navigate.clone();

    Effect::new(move |_| {
        match auth.get() {
            None => { navigate_redirect("/login", Default::default()); }
            Some(ref a) if is_token_expired(&a.token) => {
                force_logout(auth);
            }
            _ => {}
        }
    });

    let id = move || -> Option<i64> {
        params.get().get("id").and_then(|s| s.parse().ok())
    };

    let post_resource = LocalResource::new(move || {
        let post_id = id();
        async move {
            match post_id {
                Some(pid) => api::get_post(pid).await,
                None => Err("Invalid post ID".to_string()),
            }
        }
    });

    let update_action = Action::new_local(move |(title, content): &(String, String)| {
        let title = title.clone();
        let content = content.clone();
        let post_id = id();
        let token = auth.get().map(|a| a.token.clone()).unwrap_or_default();
        let navigate = navigate.clone();
        let error = error;

        async move {
            let Some(pid) = post_id else {
                error.set(Some("Invalid post ID".to_string()));
                return;
            };
            match api::update_post(pid, &title, &content, &token).await {
                Ok(post) => {
                    error.set(None);
                    navigate(&format!("/posts/{}", post.id), Default::default());
                }
                Err(e) => {
                    clear_if_unauthorized(&e, auth);
                    error.set(Some(e));
                }
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
                <A attr:class="btn btn-sm btn-outline-secondary" href=move || format!("/posts/{}", id().unwrap_or(0))>
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
                            Ok(post) => {
                                let is_owner = auth.get()
                                    .map(|a| a.user.id == post.author_id)
                                    .unwrap_or(false);

                                if !is_owner {
                                    return view! {
                                        <div class="alert alert-danger">
                                            "You don't have permission to edit this post."
                                        </div>
                                    }.into_any();
                                }

                                view! {
                                    <PostForm
                                        initial_title=post.title
                                        initial_content=post.content
                                        on_submit=on_submit
                                        loading=update_action.pending().into()
                                        error=error
                                    />
                                }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::api;
use crate::auth::{clear_if_unauthorized, use_auth};

#[component]
pub fn PostDetail() -> impl IntoView {
    let params = use_params_map();
    let auth = use_auth();
    let navigate = use_navigate();
    let delete_error = RwSignal::new(None::<String>);
    let confirming_delete = RwSignal::new(false);

    let id = move || -> Option<i64> {
        params.get().get("id").and_then(|id| id.parse().ok())
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

    let delete_action = Action::new_local(move |_: &()| {
        let post_id = id();
        let token = auth.get().map(|a| a.token.clone()).unwrap_or_default();
        let navigate = navigate.clone();

        async move {
            let Some(pid) = post_id else {
                delete_error.set(Some("Invalid post ID".to_string()));
                return;
            };
            match api::delete_post(pid, &token).await {
                Ok(_) => {
                    delete_error.set(None);
                    navigate("/", Default::default());
                }
                Err(e) => {
                    clear_if_unauthorized(&e, auth);
                    confirming_delete.set(false);
                    delete_error.set(Some(e));
                }
            }
        }
    });

    view! {
        <div class="container py-4 mx-auto" style="max-width: 780px">
            <Suspense fallback=move || view! { <p class="loading-text">"Loading post…"</p> }>
                {move || {
                    post_resource.get().map(|result| {
                        let result = (*result).clone();
                        match result {
                            Err(err) => view! {
                                <div class="alert alert-danger">{err}</div>
                            }.into_any(),
                            Ok(post) => {
                                let is_owner = move || {
                                    auth.get().map(|a| a.user.id == post.author_id).unwrap_or(false)
                                };

                                let title = post.title.clone();
                                let content = post.content.clone();
                                let post_id = post.id;
                                let author_id = post.author_id;
                                let created = post.created_at.format("%Y-%m-%d %H:%M").to_string();
                                let updated = post.updated_at.format("%Y-%m-%d %H:%M").to_string();

                                view! {
                                    <div>
                                        <A attr:class="btn btn-sm btn-outline-secondary mb-4" href="/">
                                            "← Back to posts"
                                        </A>

                                        <article>
                                            <h1 class="mb-2 fw-bold">{title}</h1>

                                            <div class="meta-bar">
                                                <div class="meta-author">
                                                    {format!("Author #{}", author_id)}
                                                </div>
                                                <div class="meta-timestamp">{format!("Created {}", created)}</div>
                                                <div class="meta-timestamp">{format!("Updated {}", updated)}</div>
                                            </div>

                                            <div class="post-content mt-4 mb-4">{content}</div>

                                            {move || delete_error.get().map(|msg| view! {
                                                <div class="alert alert-danger mt-3">{msg}</div>
                                            })}

                                            {move || {
                                                if is_owner() {
                                                    view! {
                                                        <div class="owner-actions">
                                                            <A
                                                                attr:class="btn btn-outline-primary btn-sm px-4"
                                                                href=format!("/posts/{}/edit", post_id)
                                                            >
                                                                "Edit"
                                                            </A>
                                                            {move || {
                                                                if confirming_delete.get() {
                                                                    view! {
                                                                        <span class="ms-3">
                                                                            <span class="text-danger me-2 small fw-semibold">"Are you sure?"</span>
                                                                            <button
                                                                                class="btn btn-danger btn-sm px-3"
                                                                                disabled=move || delete_action.pending().get()
                                                                                on:click=move |_| { delete_action.dispatch(()); }
                                                                            >
                                                                                {move || if delete_action.pending().get() { "Deleting…" } else { "Yes, delete" }}
                                                                            </button>
                                                                            <button
                                                                                class="btn btn-outline-secondary btn-sm px-3 ms-1"
                                                                                on:click=move |_| { confirming_delete.set(false); }
                                                                            >
                                                                                "Cancel"
                                                                            </button>
                                                                        </span>
                                                                    }.into_any()
                                                                } else {
                                                                    view! {
                                                                        <button
                                                                            class="btn btn-danger btn-sm px-4 ms-3"
                                                                            on:click=move |_| { confirming_delete.set(true); }
                                                                        >
                                                                            "Delete"
                                                                        </button>
                                                                    }.into_any()
                                                                }
                                                            }}
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <div></div> }.into_any()
                                                }
                                            }}
                                        </article>
                                    </div>
                                }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}

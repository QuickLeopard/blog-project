use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_params_map};

use crate::api;
use crate::auth::use_auth;

#[component]
pub fn PostDetail() -> impl IntoView {
    let params = use_params_map();
    let auth = use_auth();
    let navigate = use_navigate();

    let id = move || {
        params
            .get()
            .get("id")
            .and_then(|id| id.parse::<i64>().ok())
            .unwrap_or(0)
    };

    let post_resource = LocalResource::new(move || {
        let post_id = id();
        async move { api::get_post(post_id).await }
    });

    let on_delete = StoredValue::new(move |_: leptos::ev::MouseEvent| {
        let post_id = id();
        let token = auth.get().map(|a| a.token.clone()).unwrap_or_default();
        let navigate = navigate.clone();

        leptos::task::spawn_local(async move {
            if api::delete_post(post_id, &token).await.is_ok() {
                navigate("/", Default::default());
            }
        });
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
                                                            <button
                                                                class="btn btn-danger btn-sm px-4 ms-3"
                                                                on:click=move |ev| (on_delete.get_value())(ev)
                                                            >
                                                                "Delete"
                                                            </button>
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

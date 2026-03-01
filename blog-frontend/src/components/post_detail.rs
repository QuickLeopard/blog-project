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
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            {move || {
                post_resource.get().map(|result| {
                    let result = (*result).clone();
                    match result {
                        Err(err) => view! {
                            <p class="error">{err}</p>
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
                                <article>
                                    <h1>{title}</h1>
                                    <p class="text-muted">
                                        {format!("Author #{} | Created: {} | Updated: {}", author_id, created, updated)}
                                    </p>
                                    <div>{content}</div>

                                    {move || {
                                        if is_owner() {
                                            view! {
                                                <div>
                                                    <A href=format!("/posts/{}/edit", post_id)>"Edit"</A>
                                                    <button on:click=move |ev| (on_delete.get_value())(ev)>"Delete"</button>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }}
                                </article>

                                <A href="/">"Back to posts"</A>
                            }.into_any()
                        }
                    }
                })
            }}
        </Suspense>
    }
}

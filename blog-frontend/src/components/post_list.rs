use leptos::prelude::*;
use leptos_router::components::A;

use crate::api;

#[component]
pub fn PostList() -> impl IntoView {
    // Reactive signal for current page offset
    let (offset, set_offset) = signal(0i32);
    let limit = 10i32;

    // Fetches posts whenever offset changes
    let posts_resource = LocalResource::new(move || {
        let current_offset = offset.get();
        async move { api::get_posts(current_offset, limit).await }
    });

    let on_prev = move |_| {
        set_offset.update(|o| {
            *o = (*o - limit).max(0);
        });
    };

    let on_next = move |_| {
        set_offset.update(|o| {
            *o += limit;
        });
    };

    view! {
        <div class="container py-4 mx-auto" style="max-width: 760px">
            <h1 class="page-heading">"Posts"</h1>

            <Suspense fallback=move || view! { <p class="loading-text">"Loading posts…"</p> }>
                {move || {
                    posts_resource.get().map(|result| {
                        match &*result {
                            Err(err) => view! {
                                <div class="alert alert-danger">{err.clone()}</div>
                            }.into_any(),
                            Ok(data) => {
                                let post_count = data.posts.len();
                                let posts = data.posts.clone();

                                view! {
                                    <div>
                                        {posts.into_iter().map(|post| {
                                            view! {
                                                <div class="card post-card mb-3">
                                                    <div class="card-body">
                                                        <h5 class="card-title mb-1">
                                                            <A href=format!("/posts/{}", post.id)>
                                                                {post.title}
                                                            </A>
                                                        </h5>
                                                        <p class="card-text text-muted small mb-0">
                                                            {format!("Author #{} · {}",
                                                                post.author_id,
                                                                post.created_at.format("%Y-%m-%d %H:%M")
                                                            )}
                                                        </p>
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>

                                    <div class="d-flex justify-content-between align-items-center mt-3">
                                        <button
                                            class="btn btn-outline-secondary btn-sm px-3"
                                            on:click=on_prev
                                            disabled=move || offset.get() == 0
                                        >"←"</button>

                                        <span class="page-label text-muted">
                                            {move || format!("Page {}", offset.get() / limit + 1)}
                                        </span>

                                        <button
                                            class="btn btn-outline-secondary btn-sm px-3"
                                            on:click=on_next
                                            disabled=move || post_count < limit as usize
                                        >"→"</button>
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

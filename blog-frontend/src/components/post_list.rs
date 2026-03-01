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
            <h1>"Posts"</h1>

            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                {move || {
                    posts_resource.get().map(|result| {
        match &*result {
            Err(err) => view! {
                <p class="error">{err.clone()}</p>
            }.into_any(),
            Ok(data) => {
                let post_count = data.posts.len();
                let posts = data.posts.clone();  // clone since we're borrowing via &*

                view! {
                    <ul>
                        {posts.into_iter().map(|post| {
                            view! {
                                <li>
                                    <A href=format!("/posts/{}", post.id)>
                                        {post.title}
                                    </A>
                                    <span>
                                        {format!(" — author #{}, {}",
                                            post.author_id,
                                            post.created_at.format("%Y-%m-%d %H:%M")
                                        )}
                                    </span>
                                </li>
                            }
                        }).collect_view()}
                    </ul>

                    <div>
                        <button
                            on:click=on_prev
                            disabled=move || offset.get() == 0
                        >"Previous"</button>

                        <span>{move || format!(" Page {} ", offset.get() / limit + 1)}</span>

                        <button
                            on:click=on_next
                            disabled=move || post_count < limit as usize
                        >"Next"</button>
                    </div>
                }.into_any()
            }
        }
    })
                }}
            </Suspense>
        }
}

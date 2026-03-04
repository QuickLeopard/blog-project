use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::api;
use crate::auth::{clear_if_unauthorized, force_logout, is_token_expired, use_auth};
use crate::components::post_form::PostForm;

#[component]
pub fn PostCreate() -> impl IntoView {
    let auth = use_auth();
    let navigate = use_navigate();
    let error = RwSignal::new(None::<String>);

    // Clone for the effect so `navigate` stays available for create_action
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

    let create_action = Action::new_local(move |(title, content): &(String, String)| {
        let title = title.clone(); 
        let content = content.clone(); 
        let token = auth.get().map(|a| a.token.clone()).unwrap_or_default();
        let navigate = navigate.clone();  // now navigate is still in scope

        async move {
            match api::create_post(&title, &content, &token).await {
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

    let on_submit = Callback::new(move |(title, content): (String, String)| {
        create_action.dispatch((title, content));
    });

    view! {
        <div class="container py-4" style="max-width: 720px">
            <h2 class="page-heading">"New Post"</h2>
            <PostForm
                initial_title="".to_string()
                initial_content="".to_string()
                on_submit=on_submit
                loading=create_action.pending().into()
                error=error
            />
        </div>
    }
}
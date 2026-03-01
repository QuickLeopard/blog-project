use leptos::IntoView;
use leptos::component;
use leptos::prelude::Callback;
use leptos::prelude::Signal;

#[component]
pub fn PostForm(
    initial_title: String,
    initial_content: String,
    on_submit: Callback<(String, String)>, // (title, content)
    loading: Signal<bool>,
    error: Signal<Option<String>>,
) -> impl IntoView {
}

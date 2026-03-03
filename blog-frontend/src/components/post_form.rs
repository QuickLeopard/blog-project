use leptos::IntoView;
use leptos::component;
use leptos::prelude::*;

#[component]
pub fn PostForm(
    initial_title: String,
    initial_content: String,
    on_submit: Callback<(String, String)>, // (title, content)
    loading: Signal<bool>,
    error: RwSignal<Option<String>>,
) -> impl IntoView {{
    let title = RwSignal::new(initial_title);
    let content = RwSignal::new(initial_content);

    view! {
        {move || error.get().map(|msg| view! { <div class="alert alert-danger">{msg}</div> })}
        <div class="mb-3">
            <label class="form-label">"Title"</label>
            <input
                class="form-control"
                type="text"
                prop:value=move || title.get()
                on:input=move |ev| title.set(event_target_value(&ev))
            />
        </div>
        <div class="mb-3">
            <label class="form-label">"Content"</label>
            <textarea
                class="form-control"
                rows=5
                prop:value=move || content.get()
                on:input=move |ev| content.set(event_target_value(&ev))
            />
        </div>
        <button
            class="btn btn-primary"
            disabled=move || loading.get()
            on:click=move |_| { on_submit.run((title.get(), content.get())); }
        >
            {move || if loading.get() { "Saving…" } else { "Save" }}
        </button>
    }
}}

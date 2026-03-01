use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptos::*;
use leptos_router::components::*;
use leptos_router::path;
use wasm_bindgen::prelude::wasm_bindgen;

mod api;
mod auth;
mod components;
mod pages;
mod types;

use auth::provide_auth_context;
use components::navbar::Navbar;
use components::post_list::PostList;
use components::post_detail::PostDetail;
use pages::*;

#[wasm_bindgen(start)]
pub fn main() {
    mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    provide_auth_context();

    view! {
        <Router>
            <Navbar/>
            <main>
                <Routes fallback=|| "Page not found">
                    <Route path=path!("/")                view=PostList/>
                    /*<Route path=path!("/login")           view=Login/>
                    <Route path=path!("/register")        view=Register/>
                    <Route path=path!("/posts/new")       view=PostCreate/>*/
                    <Route path=path!("/posts/:id")       view=PostDetail/>
                    //<Route path=path!("/posts/:id/edit")  view=PostEdit/>
                </Routes>
            </main>
        </Router>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}

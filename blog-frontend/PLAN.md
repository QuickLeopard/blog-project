# blog-frontend — Leptos CSR Development Plan

> **Stack:** Leptos 0.7 (CSR), gloo-net, gloo-storage, leptos_router  
> **Deployment:** nginx container (Option A) — serves WASM static files, proxies `/api/*` to Actix  
> **Principles:** DRY (shared `PostForm`, shared `api.rs`), SOLID (single-responsibility per module, open/closed via context)

---

## Architecture Overview

```
Browser
  └── nginx:80
        ├── /* → serves WASM/JS/HTML from dist/
        └── /api/* → proxy → blog-server:3000 (Actix, internal network)
```

```
blog-frontend/
├── Cargo.toml
├── Trunk.toml           — build config + dev proxy
├── index.html
├── Dockerfile           — trunk build + nginx stage
├── nginx.conf           — SPA fallback + /api proxy
└── src/
    ├── lib.rs                — WASM entry point, App component, router, context providers
    ├── types.rs              — All shared structs (Post, User, requests, responses)
    ├── api.rs                — One function per HTTP endpoint (gloo-net)
    ├── auth.rs               — AuthState, localStorage persistence, context helpers
    ├── pages/
    │   ├── mod.rs
    │   ├── post_list.rs      — GET /api/posts?offset=&limit=
    │   ├── post_detail.rs    — GET /api/posts/{id}  + DELETE /api/posts/{id}
    │   ├── post_create.rs    — POST /api/posts
    │   ├── post_edit.rs      — PUT /api/posts/{id}
    │   ├── login.rs          — POST /api/auth/login
    │   └── register.rs       — POST /api/auth/register
    └── components/
        ├── mod.rs
        ├── navbar.rs         — Navigation, auth-aware links
        └── post_form.rs      — Reusable title+content form (DRY for create+edit)
```

---

## Server HTTP API Reference

All routes are under the `/api` scope on the Actix server.

| Method | Path                       | Auth     | Request body                       | Response body                              | Status |
|--------|----------------------------|----------|------------------------------------|--------------------------------------------|--------|
| POST   | `/api/auth/register`       | No       | `{ username, email, password }`    | `{ token, user: { id, username, email, created_at } }` | 201 |
| POST   | `/api/auth/login`          | No       | `{ username, password }`           | `{ token, user: { id, username, email, created_at } }` | 200 |
| GET    | `/api/posts?offset=&limit=`| No       | —                                  | `{ posts: [...], total, offset, limit }`   | 200 |
| GET    | `/api/posts/{id}`          | No       | —                                  | `{ id, title, content, author_id, created_at, updated_at }` | 200 |
| POST   | `/api/posts`               | Bearer   | `{ title, content }`               | `{ id, title, content, author_id, created_at, updated_at }` | 201 |
| PUT    | `/api/posts/{id}`          | Bearer   | `{ title, content }`               | `{ id, title, content, author_id, created_at, updated_at }` | 200 |
| DELETE | `/api/posts/{id}`          | Bearer   | —                                  | `{ message: "Post deleted!" }`             | 200 |

---

## Step 0 — Create the crate in the workspace

From inside the existing `blog-frontend/` directory:

```bash
cd blog-frontend
cargo init --lib
```

This generates `Cargo.toml` and `src/lib.rs` inside the existing folder without touching other files (like `PLAN.md`).

Then add `"blog-frontend"` to the root `Cargo.toml`:

```toml
[workspace]
members = [
    "blog-server",
    "blog-client",
    "blog-cli",
    "blog-wasm",
    "blog-frontend",
]
```

Verify it works:
```bash
cargo check -p blog-frontend
```

Now replace the generated `blog-frontend/Cargo.toml` contents with the one below and delete the placeholder code in `src/lib.rs`.

---

## Step 1 — Crate Setup

**`blog-frontend/Cargo.toml`** (replace generated file):

```toml
[package]
name = "blog-frontend"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
leptos        = { version = "0.7", features = ["csr"] }
leptos_router = "0.7"
gloo-net      = { version = "0.6", features = ["http"] }
gloo-storage  = "0.3"
serde         = { version = "1.0", features = ["derive"] }
serde_json    = "1.0"
wasm-bindgen  = "0.2"
chrono        = { version = "0.4", features = ["serde", "wasmbind"] }
```

> `chrono` needs the `wasmbind` feature in WASM targets — without it `Utc::now()` will panic at runtime.  
> Verify `gloo-net` version with `cargo search gloo-net` — if 0.6 does not exist yet, use 0.5.

**`index.html`** — minimal trunk entry point:
```html
<!DOCTYPE html>
<html>
  <head><meta charset="utf-8"/><title>Blog</title></head>
  <body></body>
</html>
```

**`Trunk.toml`** — build config + dev proxy:
```toml
[serve]
port = 8080

[[proxy]]
rewrite = "/api/"
backend = "http://localhost:3000"
```

This makes `trunk serve` proxy `/api/*` to Actix on `:3000` during development — matching the nginx behaviour in production. No CORS needed in either environment.

Install trunk: `cargo install trunk`

**Dev command:**
```bash
trunk serve --open
```

---

## Step 2 — `types.rs`

> **SRP:** one file owns all data shapes. No business logic here.

Define all structs with `#[derive(Debug, Clone, Serialize, Deserialize)]`:

```rust
// Matches blog-server/src/domain/post.rs serialization
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Matches blog-server/src/domain/user.rs serialization
// Note: password_hash is skip_serializing on server, so it is never in the JSON
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

// POST /api/auth/login — request body
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// POST /api/auth/register — request body
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

// POST /api/auth/login and /api/auth/register — response body
pub struct LoginUserResponse {
    pub token: String,
    pub user: User,
}

// POST /api/posts — request body
// PUT /api/posts/{id} — request body (same shape)
pub struct PostRequest {
    pub title: String,
    pub content: String,
}

// GET /api/posts?offset=&limit= — response body
pub struct ListPostsResponse {
    pub posts: Vec<Post>,
    pub total: i32,
    pub offset: i32,
    pub limit: i32,
}

// DELETE /api/posts/{id} — response body
pub struct DeletePostResponse {
    pub message: String,
}
```

> Note: `blog-client` cannot be reused in WASM (depends on `tonic` + tokio `full`). These are intentionally redefined here — they are trivially small.
>
> The server uses separate `CreatePostRequest` / `UpdatePostRequest` types but both have identical `{ title, content }` shape, so one `PostRequest` struct suffices here (DRY).

---

## Step 3 — `api.rs`

> **SRP + OCP:** one function per endpoint. New endpoints = new functions, nothing changes.  
> **DRY:** single `API_BASE` constant, single helper for JSON request/response + error handling.

Because nginx proxies `/api/*` to Actix, and trunk does the same in dev, the base URL is just an empty string — same origin in both environments:

```rust
const API_BASE: &str = "";   // calls /api/posts, not http://localhost:3000/api/posts
```

Functions to implement (all `async`, return `Result<T, String>`):

```rust
// POST /api/auth/login — body: { username, password } → LoginUserResponse
pub async fn login(username: &str, password: &str) -> Result<LoginUserResponse, String>

// POST /api/auth/register — body: { username, email, password } → LoginUserResponse
pub async fn register(username: &str, email: &str, password: &str) -> Result<LoginUserResponse, String>

// GET /api/posts?offset={offset}&limit={limit} → ListPostsResponse
pub async fn get_posts(offset: i32, limit: i32) -> Result<ListPostsResponse, String>

// GET /api/posts/{id} → Post
pub async fn get_post(id: i64) -> Result<Post, String>

// POST /api/posts — header: Authorization: Bearer {token}, body: { title, content } → Post
pub async fn create_post(title: &str, content: &str, token: &str) -> Result<Post, String>

// PUT /api/posts/{id} — header: Authorization: Bearer {token}, body: { title, content } → Post
pub async fn update_post(id: i64, title: &str, content: &str, token: &str) -> Result<Post, String>

// DELETE /api/posts/{id} — header: Authorization: Bearer {token} → DeletePostResponse
pub async fn delete_post(id: i64, token: &str) -> Result<DeletePostResponse, String>
```

Implementation pattern using `gloo_net::http::Request`:

```rust
// Example: login
let resp = Request::post(&format!("{}/api/auth/login", API_BASE))
    .header("Content-Type", "application/json")
    .body(serde_json::to_string(&LoginRequest { username, password }).unwrap())?
    .send()
    .await
    .map_err(|e| e.to_string())?;

if !resp.ok() {
    return Err(format!("HTTP {}", resp.status()));
}

resp.json::<LoginUserResponse>().await.map_err(|e| e.to_string())

// Example: create_post (authenticated)
let resp = Request::post(&format!("{}/api/posts", API_BASE))
    .header("Content-Type", "application/json")
    .header("Authorization", &format!("Bearer {}", token))
    .body(serde_json::to_string(&PostRequest { title, content }).unwrap())?
    .send()
    .await
    .map_err(|e| e.to_string())?;

// Example: delete_post (authenticated, no request body)
let resp = Request::delete(&format!("{}/api/posts/{}", API_BASE, id))
    .header("Authorization", &format!("Bearer {}", token))
    .send()
    .await
    .map_err(|e| e.to_string())?;
```

---

## Step 4 — `auth.rs`

> **SRP:** all auth state logic in one place.  
> **DIP:** components depend on the context abstraction, not on localStorage directly.

```rust
#[derive(Clone, Serialize, Deserialize)]
pub struct AuthState {
    pub token: String,
    pub user: User,
}

// Call once at app startup inside App component:
pub fn provide_auth_context()

// Use inside any component:
pub fn use_auth() -> RwSignal<Option<AuthState>>

// Call after successful login/register:
pub fn set_auth(state: AuthState)   // saves to localStorage + updates signal

// Call on logout:
pub fn clear_auth()                 // removes from localStorage + sets signal to None
```

Use `gloo_storage::LocalStorage` for persistence — auth survives page refresh.

---

## Step 5 — `lib.rs`

> **OCP:** adding routes does not change existing pages.

`lib.rs` is the WASM entry point — Trunk finds `wasm_bindgen(start)` here and uses it as the start function. There is no `main.rs` in a Leptos CSR crate.

Route order matters: static routes (`/posts/new`) must come **before** parameterized routes (`/posts/:id`) to avoid the wildcard matching the literal `"new"`.

```rust
use leptos::*;
use leptos_router::components::*;
use leptos_router::path;
use wasm_bindgen::prelude::wasm_bindgen;

mod types;
mod api;
mod auth;
mod pages;
mod components;

use auth::provide_auth_context;
use components::navbar::Navbar;
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
                    <Route path=path!("/login")           view=Login/>
                    <Route path=path!("/register")        view=Register/>
                    <Route path=path!("/posts/new")       view=PostCreate/>
                    <Route path=path!("/posts/:id")       view=PostDetail/>
                    <Route path=path!("/posts/:id/edit")  view=PostEdit/>
                </Routes>
            </main>
        </Router>
    }
}
```

> Note: Leptos 0.7 uses `path!()` macro and `leptos_router::components::*`. Verify exact API against 0.7 docs when implementing — the router API evolved across 0.7.x releases.

---

## Step 6 — `components/navbar.rs`

Reads `use_auth()` signal:
- **Logged out:** Login link, Register link
- **Logged in:** username display, New Post link (`/posts/new`), Logout button (calls `clear_auth()`)

No business logic — pure presentation.

---

## Step 7 — `components/post_form.rs`

> **DRY:** single form reused by both `PostCreate` and `PostEdit`.

```rust
#[component]
pub fn PostForm(
    initial_title: String,
    initial_content: String,
    on_submit: Callback<(String, String)>,  // (title, content)
    loading: Signal<bool>,
    error: Signal<Option<String>>,
) -> impl IntoView
```

Contains: controlled `<input>` for title, `<textarea>` for content, submit `<button>` (disabled while loading), error message display.

---

## Step 8 — `pages/post_list.rs`

```rust
// create_resource(|| (), |_| api::get_posts(0, 10))
// Suspense with loading fallback
// Renders list of post titles as <A href=format!("/posts/{}", post.id)> links
// Show post.author_id and post.created_at as metadata
// Previous / Next buttons updating offset signal for pagination
```

---

## Step 9 — `pages/post_detail.rs`

```rust
// Read :id param from route, parse to i64
// create_resource(move || id, |id| api::get_post(id))
// Display post title, content, author_id, created_at, updated_at
// If auth_state.user.id == post.author_id → show Edit and Delete buttons
// Delete: create_action → api::delete_post → on success navigate("/")
//   (server returns { message: "Post deleted!" } — just check success)
// Edit button: navigate to /posts/{id}/edit
```

---

## Step 10 — `pages/post_create.rs`

```rust
// If no auth token → redirect to "/login"
// create_action that calls api::create_post(title, content, token)
//   server returns the created Post with its new id
// Uses <PostForm initial_title="" initial_content="" .../>
// On success → navigate(format!("/posts/{}", post.id))
```

---

## Step 11 — `pages/post_edit.rs`

```rust
// If no auth token → redirect to "/login"
// Read :id param from route, parse to i64
// create_resource to load existing post via api::get_post(id) (pre-fill form)
// create_action that calls api::update_post(id, title, content, token)
//   server returns the updated Post
// Uses <PostForm initial_title=post.title initial_content=post.content .../>
// On success → navigate(format!("/posts/{}", id))
```

---

## Step 12 — `pages/login.rs`

```rust
// Controlled inputs: username, password
// create_action that calls api::login(username, password)
//   server returns LoginUserResponse { token, user }
// On success → set_auth(AuthState { token, user }) → navigate("/")
// Show error message on failure
// Link to /register for new users
```

---

## Step 13 — `pages/register.rs`

```rust
// Controlled inputs: username, email, password
// create_action that calls api::register(username, email, password)
//   server returns LoginUserResponse { token, user }
// On success → set_auth(AuthState { token, user }) → navigate("/")
// Show error message on failure
// Link to /login for existing users
```

---

## Step 14 — `blog-frontend/nginx.conf`

```nginx
server {
    listen 80;

    root /usr/share/nginx/html;
    index index.html;

    # Proxy API calls to the Actix backend (Docker internal hostname)
    location /api/ {
        proxy_pass http://blog-server:3000;
    }

    # SPA fallback — all unknown routes serve index.html (client-side router handles them)
    location / {
        try_files $uri $uri/ /index.html;
    }
}
```

---

## Step 15 — `blog-frontend/Dockerfile`

Two-stage build: trunk compiles WASM, nginx serves the output.

```dockerfile
# Stage 1 — build WASM
FROM rust:1.88-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev curl \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /workspace

# Copy workspace manifests (cache layer)
COPY Cargo.toml Cargo.lock ./
COPY blog-frontend/Cargo.toml ./blog-frontend/
COPY blog-client/Cargo.toml   ./blog-client/
COPY blog-cli/Cargo.toml      ./blog-cli/
COPY blog-wasm/Cargo.toml     ./blog-wasm/
COPY blog-server/Cargo.toml   ./blog-server/

# Copy sources
COPY blog-frontend/src        ./blog-frontend/src
COPY blog-frontend/index.html ./blog-frontend/index.html
COPY blog-frontend/Trunk.toml ./blog-frontend/Trunk.toml
# Stub sources for other workspace members (needed for Cargo to resolve the workspace)
COPY blog-client/src          ./blog-client/src
COPY blog-server/src          ./blog-server/src
COPY blog-server/build.rs     ./blog-server/
COPY blog-server/proto        ./blog-server/proto
COPY blog-client/proto        ./blog-client/proto
COPY blog-client/build.rs     ./blog-client/
COPY blog-cli/src             ./blog-cli/src
COPY blog-wasm/src            ./blog-wasm/src

# Build from the directory that contains index.html
WORKDIR /workspace/blog-frontend
RUN trunk build --release --dist /dist

# Stage 2 — serve with nginx
FROM nginx:alpine
COPY --from=builder /dist /usr/share/nginx/html
COPY blog-frontend/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
```

---

## Step 16 — `docker-compose.yml` additions

Add the `blog-frontend` service. No changes to existing services.

```yaml
  blog-frontend:
    build:
      context: .
      dockerfile: ./blog-frontend/Dockerfile
    container_name: blog-frontend
    ports:
      - "80:80"
    depends_on:
      - blog-server
```

Full request flow in Docker:
```
Browser:80 → nginx → /api/* → blog-server:3000 (internal)
                   → /*     → dist/index.html + WASM
```

---

## SOLID Checklist

| Principle | Applied where |
|-----------|---------------|
| **S** Single Responsibility | Each file owns one concern: types, api, auth, one page per route |
| **O** Open/Closed | New pages = new files; router, navbar extended without modifying existing pages |
| **L** Liskov | N/A (no inheritance in Rust) |
| **I** Interface Segregation | `PostForm` callback takes only `(String, String)` — callers are not forced to know about tokens or IDs |
| **D** Dependency Inversion | Components depend on `use_auth()` context abstraction, not on `LocalStorage` directly |

## DRY Checklist

| Duplication avoided | Solution |
|---------------------|----------|
| Title+content form in create AND edit | Shared `PostForm` component |
| `CreatePostRequest` / `UpdatePostRequest` (identical shape) | Single `PostRequest` struct |
| HTTP boilerplate in every API call | Single helper in `api.rs` + `API_BASE` constant |
| Auth token extraction in every protected page | `use_auth()` helper from `auth.rs` |
| Error display markup | Inline in `PostForm`; pages delegate error signal to it |
| CORS configuration | Eliminated entirely — nginx proxy + trunk proxy make all calls same-origin |
| Hardcoded backend URL | `API_BASE = ""` works in dev (trunk proxy) and prod (nginx proxy) without change |

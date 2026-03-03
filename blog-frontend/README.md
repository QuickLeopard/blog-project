# blog-frontend

Leptos 0.7 single-page application compiled to WebAssembly, with Bootstrap 5 styling.

## Features

- User registration and login with JWT token persistence (localStorage)
- Post list with paginated card layout (← Page N / M →)
- Post detail view with author metadata, edit, and delete actions
- Create and edit posts with a reusable form component
- Auth-aware navbar and route protection
- Friendly error messages parsed from server JSON responses

## Architecture

```
src/
├── lib.rs              App component, router, WASM entry point
├── types.rs            Shared structs (Post, User, requests, responses)
├── api.rs              Async HTTP functions (one per endpoint)
├── auth.rs             AuthState, localStorage persistence, context helpers
├── pages/
│   ├── login.rs        Login form with error handling
│   ├── register.rs     Registration form
│   ├── post_create.rs  New post page (auth-protected)
│   └── post_edit.rs    Edit post page (pre-fills from API)
└── components/
    ├── navbar.rs       Auth-aware navigation bar
    ├── post_form.rs    Reusable title/content form with loading spinner
    ├── post_list.rs    Paginated post cards (home page)
    └── post_detail.rs  Single post view with owner actions
```

## Prerequisites

- Rust with `wasm32-unknown-unknown` target
- [Trunk](https://trunkrs.dev)

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Running (development)

Requires blog-server running on `localhost:3000` (see root README for options).

```bash
cd blog-frontend
trunk serve
```

Opens on http://localhost:8080 with hot reload. API calls are proxied to the backend via `Trunk.toml`:

```toml
[[proxy]]
rewrite = "/api/"
backend = "http://localhost:3000"
```

## Building for production

```bash
trunk build --release
```

Output goes to `dist/` — static files ready to be served by Nginx or any static file server. The production Docker image uses Nginx with SPA fallback (`nginx.conf`).

## Styling

Bootstrap 5.3.2 is loaded from CDN in `index.html`. Custom overrides live in `style.css` (card hover effects, meta-bar layout, accent colors, loading animations).

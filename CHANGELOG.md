# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `blog-client/src/error.rs` — typed `BlogClientError` enum with variants:
  `Http`, `Transport`, `NotFound`, `Unauthorized`, `Conflict`,
  `InvalidRequest`, `Internal`
- `CURRENT-ISSUES.md` — audit of codebase gaps against `todo-plan.html`
- Priority 1 doc comments on `DomainError`, `Post`, `User`,
  `ListPostsResponse`, `BlogService` trait, and `BlogClient`
- Named constants replacing all hardcoded literals:
  `TOKEN_TTL_HOURS`, `DB_MAX/MIN_CONNECTIONS`, `DB_ACQUIRE_TIMEOUT_SECS`,
  `HTTP_PORT`, `GRPC_PORT`, `CORS_MAX_AGE_SECS`, `DEFAULT_PAGE_LIMIT/OFFSET`,
  `TITLE/CONTENT_MAX_LEN`, `HTTP_TIMEOUT_SECS`, `DEFAULT_HTTP/GRPC_ADDR`,
  `DEFAULT_LIST_LIMIT`, `TOKEN_FILE`

### Changed
- `BlogService` trait and all implementations now return
  `Result<T, BlogClientError>` instead of `anyhow::Result`
- `HttpClient` maps HTTP status codes to typed `BlogClientError` variants
  (401/403 → `Unauthorized`, 404 → `NotFound`, 409 → `Conflict`, etc.)
  and parses JSON `{"error": "..."}` body for friendly messages
- `HttpClient` adds a 30-second request timeout via `reqwest::Client::builder`
- `BlogGrpcClient` maps `tonic::Status` codes to `BlogClientError` variants;
  extracted `authed_request` and `grpc_post` helpers to reduce duplication
- `blog-cli`: `--token` is now `Option<String>` on all authenticated commands —
  auto-saved to `.blog_token` on `register`/`login` and auto-loaded on
  subsequent commands; explicit `--token` overrides the file
- `blog-cli`: friendly `print_error()` handler replaces raw `anyhow` output
- `blog-cli`: improved output formatting with ✓ checkmarks and structured
  `GetPost` display (title, author, timestamps, separator, content)
- `blog-cli/README.md`: added "Управление токеном" section documenting
  auto-save/load behaviour and token priority order; updated all examples
- `README.md`: removed stale `blog-wasm/` entry from architecture tree
- `CHANGELOG.md`: corrected `blog-wasm` → `blog-frontend` in v0.1.0 entry

## [0.2.0] - 2026-03-04

### Added
- Full Leptos 0.7 CSR SPA (`blog-frontend`) replacing planned `blog-wasm`:
  - Pages: Login, Register, PostList (paginated), PostDetail, PostCreate, PostEdit
  - Components: Navbar, PostForm, PostDetail, PostList
  - `api.rs` — gloo-net HTTP layer with typed error handling
  - `auth.rs` — JWT stored in `localStorage`; `use_auth()` reactive signal
  - Custom CSS (`style.css`) with CSS custom properties, animations, and
    Bootstrap 5.3 overrides
  - Dockerfile with multi-stage Trunk build + Nginx production serving
  - `Trunk.toml` with `/api/` proxy to backend
- `PostEdit` page with `LocalResource` pre-fill and `Action`-based update
- GitHub Actions CI workflow with build, test, and Clippy report steps
- Unit tests for `infrastructure/hash.rs` (6 cases)
- Unit tests for `infrastructure/jwt.rs` (7 cases)
- In-memory repositories (`InMemoryPostRepository`, `InMemoryUserRepository`)
  for testing without a database
- `blog_service.count_posts()` to return total post count for pagination
- Comprehensive README files for root project and all 4 crates (Russian)
- Docker Compose with PostgreSQL + blog-server + blog-frontend (Nginx)

### Fixed
- Pagination showed "Page 1 / 1" regardless of total posts — `http_public.rs`
  now calls `count_posts()` instead of returning `posts.len()`
- Frontend runtime WASM panic on login/register — `expect_context` was called
  inside async `Action`; fixed by capturing `RwSignal` synchronously
- Frontend API 404 errors with `trunk serve` — `Trunk.toml` proxy backend
  set to `http://localhost:3000/api/` to preserve the `/api/` prefix
- Docker image missing custom CSS — `Dockerfile` now COPYs `style.css`
- `blog-client` unused imports in `grpc_client.rs` causing build warnings

### Changed
- HTTP server port changed from 8080 to 3000
- CORS configuration extracted to `build_cors()` helper
- `get_posts` pagination default values use named constants

## [0.1.0] - 2024-01-18

### Added
- Cargo workspace with 4 crates: blog-server, blog-client, blog-cli, blog-frontend
- gRPC proto definitions (blog.proto) with BlogService
- Clean architecture structure: domain, application, data, infrastructure, presentation
- Domain models: User, Post with request DTOs and custom errors (thiserror)
- SQL migrations for users and posts tables
- PostgreSQL database integration with sqlx
- Connection pooling (min 5, max 20 connections)
- Automatic migrations runner
- JWT infrastructure module
- Logging infrastructure with tracing
- Docker Compose setup with PostgreSQL 16
- Multi-stage Dockerfile for blog-server
- Environment variables configuration (.env.example)
- Health check endpoint (GET /api/health)
- actix-web HTTP server on port 8080
- Database connection and migration on startup
- HTTP API endpoints for user authentication (register, login)
- HTTP API endpoints for posts (get by id, list with pagination)
- RegisterUserRequest and LoginUserResponse DTOs
- InMemoryPostRepository for testing
- Pagination query parameters support
- API routes under /api scope
- BlogService repository pattern with dependency injection

### Infrastructure
- Docker support with docker-compose.yml
- PostgreSQL 16 in Docker container
- Environment variable parameterization
- .gitignore configuration

## [0.0.1] - 2024-01-01

### Added
- Initial project setup
- Repository structure

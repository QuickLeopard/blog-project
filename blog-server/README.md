# blog-server

Actix-Web REST API and Tonic gRPC server for the blog platform.

## Responsibilities

- User registration and login with Argon2 password hashing
- JWT-based authentication via middleware
- CRUD operations for blog posts (owner-scoped update/delete)
- PostgreSQL persistence with SQLx (auto-migrations on startup)
- gRPC service (Tonic) mirroring all HTTP endpoints

## Architecture

```
presentation/
├── http_public.rs      Public endpoints (login, register, get posts)
├── http_protected.rs   JWT-protected endpoints (create, update, delete posts)
├── middleware.rs        AuthenticatedUser extractor (JWT validation)
└── grpc_service.rs     gRPC BlogService implementation

application/
├── auth_service.rs     Register, login, token generation
└── blog_service.rs     Post CRUD logic

data/
├── post_repository_trait.rs    PostRepository trait (async)
├── user_repository_trait.rs    UserRepository trait (async)
├── db_post_repository.rs       PostgreSQL implementation
├── db_user_repository.rs       PostgreSQL implementation
├── in_memory_post_repository.rs  In-memory impl (for testing)
└── in_memory_user_repository.rs  In-memory impl (for testing)

infrastructure/
├── database.rs         Connection pool + migration runner
├── jwt.rs              JwtService (generate/verify tokens)
├── hash.rs             Argon2 hash_password / verify_password
├── app_state.rs        Shared AppState struct
└── logging.rs          Tracing setup

domain/
├── post.rs             Post struct, request/response types
├── user.rs             User struct, auth request/response types
└── error.rs            DomainError enum with HTTP status mapping
```

## Configuration

| Env Variable | Required | Default | Description |
|---|---|---|---|
| `DATABASE_URL` | Yes | `postgres://localhost/blog` | PostgreSQL connection string |
| `SECRET_TOKEN` | Yes | — | JWT signing secret |
| `RUST_LOG` | No | `info` | Log level filter |

## Running

```bash
DATABASE_URL=postgres://blog_user:blog_password@localhost:5432/blog \
SECRET_TOKEN=my-dev-secret \
cargo run -p blog-server
```

Starts HTTP on `0.0.0.0:3000` and gRPC on `0.0.0.0:50051`. Migrations run automatically.

## Database

PostgreSQL 16+ required. Two tables created by migrations in `migrations/`:

- **users** — `id`, `username` (unique), `email` (unique), `password_hash`, `created_at`
- **posts** — `id`, `title`, `content`, `author_id` (FK → users), `created_at`, `updated_at`

## Testing

```bash
cargo test -p blog-server
```

Unit tests exist for `hash_password`/`verify_password`. In-memory repository implementations are available for testing services without a database.

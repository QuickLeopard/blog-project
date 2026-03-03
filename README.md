# Blog Project

A full-stack blog platform built with Rust, featuring an Actix-Web/gRPC backend, a Leptos WASM frontend, and a CLI client.

## Architecture

```
blog-project/
├── blog-server/      Actix-Web REST API + Tonic gRPC server
├── blog-frontend/    Leptos 0.7 SPA (compiled to WASM, served by Trunk)
├── blog-client/      Shared client library (HTTP + gRPC transports)
├── blog-cli/         Command-line client using blog-client
└── blog-wasm/        Standalone WASM utility module
```

**Tech stack:** Rust 2024 edition, Actix-Web 4, Tonic (gRPC), Leptos 0.7 (CSR), SQLx + PostgreSQL, Argon2 password hashing, JWT authentication, Bootstrap 5 UI.

## Prerequisites

- **Rust** (stable, 1.85+) with `wasm32-unknown-unknown` target
- **Docker** and **Docker Compose**
- **Trunk** (`cargo install trunk`) — for local frontend development
- **Protobuf compiler** (`protoc`) — for gRPC code generation

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Environment Setup

Copy the example env file and edit as needed:

```bash
cp .env.example .env
```

Required variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `POSTGRES_USER` | Database user | `blog_user` |
| `POSTGRES_PASSWORD` | Database password | `blog_password` |
| `POSTGRES_DB` | Database name | `blog` |
| `DATABASE_URL` | Full Postgres connection string | built from above |
| `SECRET_TOKEN` | JWT signing secret (required) | none |
| `RUST_LOG` | Log level | `info` |

---

## Running the Project

### Option 1: Everything in Docker

The simplest way — no local Rust toolchain needed for running.

```bash
docker compose up --build
```

This starts:
- **PostgreSQL** on port `5432`
- **blog-server** on port `3000` (HTTP) and `50051` (gRPC)
- **blog-frontend** on port `8080` (Nginx serving WASM + proxying `/api/` to blog-server)

Open http://localhost:8080 in your browser.

### Option 2: Backend in Docker, Frontend local (recommended for development)

Best for frontend development — Trunk provides hot reload on code changes.

**1. Start Postgres and the backend:**

```bash
docker compose up postgres blog-server
```

**2. Run the frontend with Trunk:**

```bash
cd blog-frontend
trunk serve
```

Trunk starts on http://localhost:8080 and proxies `/api/` requests to `localhost:3000` (configured in `Trunk.toml`).

### Option 3: Everything local (no Docker)

**1. Start PostgreSQL locally** (e.g. via Homebrew, system package, or a standalone container):

```bash
# standalone Postgres container (if you prefer)
docker run -d --name blog-pg -p 5432:5432 \
  -e POSTGRES_USER=blog_user \
  -e POSTGRES_PASSWORD=blog_password \
  -e POSTGRES_DB=blog \
  postgres:16-alpine
```

**2. Run the backend:**

```bash
DATABASE_URL=postgres://blog_user:blog_password@localhost:5432/blog \
SECRET_TOKEN=my-dev-secret \
cargo run -p blog-server
```

The server runs migrations automatically on startup. HTTP API on `localhost:3000`, gRPC on `localhost:50051`.

**3. Run the frontend:**

```bash
cd blog-frontend
trunk serve
```

Open http://localhost:8080.

---

## Using the CLI (`blog-cli`)

The CLI communicates with the backend via HTTP or gRPC. It requires a running blog-server.

### Build

```bash
cargo build -p blog-cli
```

### Protocol selection

Every command requires either `--http` or `--grpc`:

```bash
# HTTP (default server: 127.0.0.1:3000)
cargo run -p blog-cli -- --http <command>

# gRPC (default server: 127.0.0.1:50051)
cargo run -p blog-cli -- --grpc <command>

# Custom server address
cargo run -p blog-cli -- --http --server 192.168.1.10:3000 <command>
```

### Commands

**Register a new user:**

```bash
cargo run -p blog-cli -- --http register \
  --username alice \
  --email alice@example.com \
  --password secret123
```

**Login (returns a JWT token):**

```bash
cargo run -p blog-cli -- --http login \
  --username alice \
  --password secret123
```

Save the token from the output for authenticated commands.

**Create a post:**

```bash
cargo run -p blog-cli -- --http create-post \
  --title "My First Post" \
  --content "Hello, world!" \
  --token "<your-jwt-token>"
```

**List posts:**

```bash
cargo run -p blog-cli -- --http list-posts --offset 0 --limit 10
```

**Get a single post:**

```bash
cargo run -p blog-cli -- --http get-post 1
```

**Update a post:**

```bash
cargo run -p blog-cli -- --http update-post \
  --id 1 \
  --title "Updated Title" \
  --content "Updated content" \
  --token "<your-jwt-token>"
```

**Delete a post:**

```bash
cargo run -p blog-cli -- --http delete-post \
  --id 1 \
  --token "<your-jwt-token>"
```

---

## API Endpoints

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| GET | `/api/health` | No | Health check |
| POST | `/api/auth/register` | No | Register user |
| POST | `/api/auth/login` | No | Login, returns JWT |
| GET | `/api/posts` | No | List posts (paginated) |
| GET | `/api/posts/:id` | No | Get single post |
| POST | `/api/posts` | JWT | Create post |
| PUT | `/api/posts/:id` | JWT | Update post (owner only) |
| DELETE | `/api/posts/:id` | JWT | Delete post (owner only) |

---

## Testing

```bash
# Run all tests
cargo test --workspace

# Run server tests only
cargo test -p blog-server

# Run specific test module
cargo test -p blog-server -- hash::tests
```

---

## Project Structure

See individual README files for details:

- [blog-server/README.md](blog-server/README.md) — Backend API server
- [blog-frontend/README.md](blog-frontend/README.md) — Leptos WASM frontend
- [blog-client/README.md](blog-client/README.md) — Shared client library
- [blog-cli/README.md](blog-cli/README.md) — Command-line client

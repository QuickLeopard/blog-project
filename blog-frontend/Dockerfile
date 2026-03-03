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
COPY blog-frontend/style.css  ./blog-frontend/style.css
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
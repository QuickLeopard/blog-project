FROM rust:1.88-slim as builder

WORKDIR /workspace

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY blog-server/Cargo.toml ./blog-server/
COPY blog-client/Cargo.toml ./blog-client/
COPY blog-cli/Cargo.toml ./blog-cli/
COPY blog-frontend/Cargo.toml ./blog-frontend/

COPY blog-server/build.rs ./blog-server/
COPY blog-server/proto ./blog-server/proto
COPY blog-client/proto ./blog-client/proto
COPY blog-server/src ./blog-server/src
COPY blog-client/src ./blog-client/src
COPY blog-cli/src ./blog-cli/src
COPY blog-client/build.rs ./blog-client/
COPY blog-server/migrations ./blog-server/migrations
COPY blog-frontend/src      ./blog-frontend/src

RUN cargo build --release -p blog-server

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /workspace/target/release/blog-server /app/blog-server
COPY --from=builder /workspace/blog-server/migrations /app/migrations

EXPOSE 3000

CMD ["/app/blog-server"]

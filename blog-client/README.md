# blog-client

Shared Rust client library for communicating with the blog-server. Provides both HTTP and gRPC transports behind a common trait.

## Design

```
src/
├── traits.rs        BlogService trait (async, Send + Sync)
├── http_client.rs   HTTP implementation using reqwest
├── grpc_client.rs   gRPC implementation using tonic
├── blog_client.rs   BlogClient wrapper (owns a Box<dyn BlogService>)
├── post.rs          Post, CreatePostRequest, UpdatePostRequest, ListPostsResponse
├── user.rs          User, LoginRequest, RegisterUserRequest, LoginUserResponse
└── lib.rs           Module exports
```

## BlogService Trait

Both transports implement the same trait:

```rust
#[async_trait]
pub trait BlogService: Send + Sync {
    async fn login_user(&self, username: String, password: String) -> Result<LoginUserResponse>;
    async fn register_user(&self, username: String, email: String, password: String) -> Result<LoginUserResponse>;
    async fn create_post(&self, title: String, content: String, token: String) -> Result<Post>;
    async fn update(&self, id: i64, title: String, content: String, token: String) -> Result<Post>;
    async fn delete(&self, id: i64, token: String) -> Result<bool>;
    async fn get_post(&self, id: i64) -> Result<Post>;
    async fn get_posts(&self, offset: i32, limit: i32) -> Result<Vec<Post>>;
}
```

## Usage

```rust
use blog_client::blog_client::BlogClient;
use blog_client::http_client::HttpClient;

let transport = HttpClient::new("http://127.0.0.1:3000".into());
let client = BlogClient::new(Box::new(transport));

let posts = client.get_posts(0, 10).await?;
```

For gRPC:

```rust
use blog_client::grpc_client::BlogGrpcClient;

let transport = BlogGrpcClient::connect("http://127.0.0.1:50051".into()).await?;
let client = BlogClient::new(Box::new(transport));
```

## Dependencies

- `reqwest` — HTTP transport
- `tonic` / `prost` — gRPC transport + protobuf
- `chrono` — timestamp handling
- `async-trait` — async trait support

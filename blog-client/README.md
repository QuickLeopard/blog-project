# blog-client

Общая клиентская библиотека на Rust для взаимодействия с blog-server. Предоставляет HTTP и gRPC транспорты за единым трейтом.

## Дизайн

```
src/
├── traits.rs        Трейт BlogService (async, Send + Sync)
├── http_client.rs   HTTP-реализация на reqwest
├── grpc_client.rs   gRPC-реализация на tonic
├── blog_client.rs   Обёртка BlogClient (владеет Box<dyn BlogService>)
├── post.rs          Post, CreatePostRequest, UpdatePostRequest, ListPostsResponse
├── user.rs          User, LoginRequest, RegisterUserRequest, LoginUserResponse
└── lib.rs           Экспорт модулей
```

## Трейт BlogService

Оба транспорта реализуют один и тот же трейт:

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

## Использование

```rust
use blog_client::blog_client::BlogClient;
use blog_client::http_client::HttpClient;

let transport = HttpClient::new("http://127.0.0.1:3000".into());
let client = BlogClient::new(Box::new(transport));

let posts = client.get_posts(0, 10).await?;
```

Для gRPC:

```rust
use blog_client::grpc_client::BlogGrpcClient;

let transport = BlogGrpcClient::connect("http://127.0.0.1:50051".into()).await?;
let client = BlogClient::new(Box::new(transport));
```

## Зависимости

- `reqwest` — HTTP-транспорт
- `tonic` / `prost` — gRPC-транспорт + protobuf
- `chrono` — работа с временными метками
- `async-trait` — поддержка async-трейтов

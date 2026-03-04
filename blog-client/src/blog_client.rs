use crate::error::BlogClientError;
use crate::post::Post;
use crate::traits::BlogService;
use crate::user::LoginUserResponse;

/// High-level client facade for the blog API.
///
/// `BlogClient` wraps any type that implements [`BlogService`] — currently
/// [`crate::http_client::HttpClient`] (REST) or
/// [`crate::grpc_client::BlogGrpcClient`] (gRPC) — and exposes a stable public
/// API that is independent of the underlying transport.
///
/// Consumers (such as the CLI) should interact with `BlogClient` exclusively
/// and never call the inner transport types directly. This keeps transport
/// selection isolated to the startup/configuration phase.
///
/// # Example
/// ```rust,ignore
/// let transport = Box::new(HttpClient::new("http://127.0.0.1:3000".into()));
/// let client = BlogClient::new(transport);
/// let response = client.login_user("alice".into(), "secret".into()).await?;
/// ```
pub struct BlogClient {
    transport: Box<dyn BlogService>,
}

impl BlogClient {
    /// Create a new `BlogClient` backed by the given transport.
    ///
    /// Pass a boxed [`HttpClient`] or [`BlogGrpcClient`] depending on the
    /// desired protocol.
    pub fn new(transport: Box<dyn BlogService>) -> Self {
        Self { transport }
    }

    /// Authenticate an existing user. See [`BlogService::login_user`].
    pub async fn login_user(
        &self,
        username: String,
        password: String,
    ) -> Result<LoginUserResponse, BlogClientError> {
        self.transport.login_user(username, password).await
    }

    /// Register a new user account. See [`BlogService::register_user`].
    pub async fn register_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<LoginUserResponse, BlogClientError> {
        self.transport
            .register_user(username, email, password)
            .await
    }

    /// Create a new post. See [`BlogService::create_post`].
    pub async fn create_post(
        &self,
        title: String,
        content: String,
        token: String,
    ) -> Result<Post, BlogClientError> {
        self.transport.create_post(title, content, token).await
    }

    /// Delete a post by id. See [`BlogService::delete`].
    pub async fn delete_post(
        &self,
        id: i64,
        token: String,
    ) -> Result<bool, BlogClientError> {
        self.transport.delete(id, token).await
    }

    /// Update an existing post. See [`BlogService::update`].
    pub async fn update_post(
        &self,
        id: i64,
        title: String,
        content: String,
        token: String,
    ) -> Result<Post, BlogClientError> {
        self.transport.update(id, title, content, token).await
    }

    /// Fetch a single post by id. See [`BlogService::get_post`].
    pub async fn get_post(&self, id: i64) -> Result<Post, BlogClientError> {
        self.transport.get_post(id).await
    }

    /// Fetch a paginated list of posts. See [`BlogService::get_posts`].
    pub async fn get_posts(
        &self,
        offset: i32,
        limit: i32,
    ) -> Result<Vec<Post>, BlogClientError> {
        self.transport.get_posts(offset, limit).await
    }
}

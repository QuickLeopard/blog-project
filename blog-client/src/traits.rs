use async_trait::async_trait;

use crate::error::BlogClientError;
use crate::post::Post;
use crate::user::LoginUserResponse;

/// Transport-agnostic interface for all blog API operations.
///
/// Both [`crate::http_client::HttpClient`] (REST over HTTP) and
/// [`crate::grpc_client::BlogGrpcClient`] (gRPC) implement this trait, allowing
/// [`crate::blog_client::BlogClient`] and the CLI to switch transports without
/// changing any business logic.
///
/// All methods are `async` and return `Result<T, BlogClientError>` so callers
/// can match on specific error variants (e.g. [`BlogClientError::Unauthorized`],
/// [`BlogClientError::NotFound`]) instead of relying on opaque `anyhow` errors.
#[async_trait]
pub trait BlogService: Send + Sync {
    /// Create a new post on behalf of the authenticated user.
    ///
    /// `token` must be a valid JWT obtained from [`login_user`] or
    /// [`register_user`]. Returns the created [`Post`] with its server-assigned
    /// `id` and timestamps.
    async fn create_post(
        &self,
        title: String,
        content: String,
        token: String,
    ) -> Result<Post, BlogClientError>;

    /// Delete the post with the given `id`.
    ///
    /// The server enforces ownership — only the post's author may delete it.
    /// Returns `Ok(true)` on success. Returns [`BlogClientError::Unauthorized`]
    /// if the token is invalid or the caller is not the author.
    async fn delete(&self, id: i64, token: String) -> Result<bool, BlogClientError>;

    /// Replace the title and content of an existing post.
    ///
    /// Both fields are required; partial updates are not currently supported.
    /// The server enforces ownership — only the post's author may update it.
    /// Returns the updated [`Post`].
    async fn update(
        &self,
        id: i64,
        title: String,
        content: String,
        token: String,
    ) -> Result<Post, BlogClientError>;

    /// Authenticate an existing user and return a JWT.
    ///
    /// On success the returned [`LoginUserResponse`] contains a JWT `token` and
    /// the user's profile. The token should be stored (e.g. in `.blog_token`)
    /// and passed to authenticated methods.
    async fn login_user(
        &self,
        username: String,
        password: String,
    ) -> Result<LoginUserResponse, BlogClientError>;

    /// Register a new user account and return a JWT.
    ///
    /// Equivalent to [`login_user`] in its return value — the user is
    /// automatically authenticated after registration. Returns
    /// [`BlogClientError::Conflict`] if the username or email is already taken.
    async fn register_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<LoginUserResponse, BlogClientError>;

    /// Fetch a single post by its numeric `id`.
    ///
    /// No authentication required. Returns [`BlogClientError::NotFound`] if
    /// the post does not exist.
    async fn get_post(&self, id: i64) -> Result<Post, BlogClientError>;

    /// Fetch a paginated list of posts ordered by creation date (newest first).
    ///
    /// - `offset`: number of posts to skip (0-based).
    /// - `limit`: maximum number of posts to return.
    ///
    /// No authentication required. Returns an empty `Vec` when there are no
    /// posts in the requested range.
    async fn get_posts(&self, offset: i32, limit: i32) -> Result<Vec<Post>, BlogClientError>;
}

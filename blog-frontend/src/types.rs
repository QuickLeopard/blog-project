use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

// Matches blog-server/src/domain/post.rs serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Matches blog-server/src/domain/user.rs serialization
// Note: password_hash is skip_serializing on server, so it is never in the JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

// POST /api/auth/login — request body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// POST /api/auth/register — request body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

// POST /api/auth/login and /api/auth/register — response body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUserResponse {
    pub token: String,
    pub user: User,
}

// POST /api/posts — request body
// PUT /api/posts/{id} — request body (same shape)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRequest {
    pub title: String,
    pub content: String,
}

// GET /api/posts?offset=&limit= — response body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPostsResponse {
    pub posts: Vec<Post>,
    pub total: i32,
    pub offset: i32,
    pub limit: i32,
}

// DELETE /api/posts/{id} — response body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePostResponse {
    pub message: String,
}

// Error response from server: { "error": "...", "status": 401 }
#[derive(Debug, Clone, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

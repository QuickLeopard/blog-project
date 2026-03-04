use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

/// Core domain entity representing a blog post stored in the database.
///
/// `author_id` is a foreign key referencing `users.id`. The display name of
/// the author is not embedded here; it must be joined from the `users` table
/// when needed (see the `TODO` in `grpc_service.rs` for `author_username`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    /// Foreign key to `users.id`. Used for ownership checks before allowing
    /// update or delete operations.
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Post {
    /// Convenience constructor used in tests and in-memory repository
    /// implementations. Production code receives `Post` values deserialized
    /// from SQLx query results via `sqlx::FromRow`.
    #[allow(dead_code)]
    pub fn new(
        id: i64,
        title: String,
        content: String,
        author_id: i64,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            title,
            content,
            author_id,
            created_at,
            updated_at,
        }
    }
}

/// Deserialized from the HTTP request body for `POST /api/posts`.
#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

/// Deserialized from the HTTP request body for `PUT /api/posts/:id`.
/// Both fields are currently required. Partial updates (title-only or
/// content-only) are not yet supported; see CURRENT-ISSUES.md issue #3.
#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: String,
    pub content: String,
}

/// Serialized as the JSON body for `GET /api/posts` (paginated list).
///
/// `total` is the **full count of all posts in the database**, not just the
/// number of posts returned in this page. Frontend pagination controls should
/// compute `total_pages = ceil(total / limit)` using this value.
#[derive(Debug, Serialize)]
pub struct ListPostsResponse {
    pub posts: Vec<Post>,
    /// Total number of posts across all pages. Use this — not `posts.len()` —
    /// to calculate the page count.
    pub total: i32,
    pub offset: i32,
    pub limit: i32,
}

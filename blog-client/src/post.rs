use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: String,
    pub updated_at: String,
}

impl Post {
    pub fn new(
        id: i64,
        title: String,
        content: String,
        author_id: i64,
        created_at: String,
        updated_at: String,
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

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ListPostsResponse {
    pub posts: Vec<Post>,
    pub total: i32,
    pub offset: i32,
    pub limit: i32,
}

use crate::domain::post::Post;
use async_trait::async_trait;

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create(
        &self,
        title: String,
        content: String,
        author_id: i64,
    ) -> Result<Post, sqlx::Error>;
    async fn find_by_id(&self, id: i64) -> Result<Post, sqlx::Error>;
    async fn update(
        &self,
        id: i64,
        title: String,
        content: String,
        author_id: i64,
    ) -> Result<Post, sqlx::Error>;
    async fn delete(&self, id: i64, author_id: i64) -> Result<bool, sqlx::Error>;
    async fn list(&self, offset: i64, limit: i64) -> Result<Vec<Post>, sqlx::Error>;
    async fn count(&self) -> Result<i64, sqlx::Error>;
}

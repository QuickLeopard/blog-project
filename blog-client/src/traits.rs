use async_trait::async_trait;

use crate::post::Post;

#[async_trait]
pub trait BlogService: Send + Sync {
    /*async fn create(
        &self,
        title: String,
        content: String,
        author_id: i64,
    ) -> anyhow::Result<()>;
    //async fn find_by_id(&self, id: i64) -> anyhow::Result<()>;
    async fn update(
        &self,
        id: i64,
        title: String,
        content: String,
        author_id: i64,
    ) -> anyhow::Result<()>;
    async fn delete(&self, id: i64, author_id: i64) -> anyhow::Result<()>;*/
    async fn get_post(&self, id: i64) -> anyhow::Result<Post>;
    async fn get_posts(&self, offset: i32, limit: i32) -> anyhow::Result<Vec<Post>>;
    async fn count_posts(&self) -> anyhow::Result<i32>;
}

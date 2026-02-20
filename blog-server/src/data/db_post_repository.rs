
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::post::Post;
use crate::data::PostRepository;

pub struct DBPostRepository {
    pool: Pool<Postgres>,
}

impl DBPostRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

}

#[async_trait]
impl PostRepository for DBPostRepository {
    async fn create(&self, title: String, content: String, author_id: i64) -> Result<Post, sqlx::Error> {
        todo!("Implement post creation")
    }

    async fn find_by_id(&self, id: i64) -> Result<Post, sqlx::Error> {
        todo!("Implement find by id")
    }

    async fn update(&self, id: i64, title: String, content: String, author_id: i64) -> Result<Post, sqlx::Error> {
        todo!("Implement post update")
    }

    async fn delete(&self, id: i64, author_id: i64) -> Result<bool, sqlx::Error> {
        todo!("Implement post deletion")
    }

    async fn list(&self, offset: i64, limit: i64) -> Result<Vec<Post>, sqlx::Error> {
        todo!("Implement list posts")
    }

    async fn count(&self) -> Result<i64, sqlx::Error> {
        todo!("Implement count posts")
    }
}

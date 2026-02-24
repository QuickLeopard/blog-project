use std::sync::Arc;

use crate::data::PostRepository;
use crate::domain::{error::DomainError, post::Post};

//#[derive(Clone)]
pub struct BlogService {
    post_repository: Arc<dyn PostRepository>,
}

impl BlogService {
    pub fn new(post_repository: Arc<dyn PostRepository>) -> Self {
        Self { post_repository }
    }

    pub async fn create_post(
        &self,
        title: String,
        content: String,
        author_id: i64,
    ) -> Result<Post, DomainError> {
        Ok(self
            .post_repository
            .create(title, content, author_id)
            .await?)
    }

    pub async fn update_post(
        &self,
        id: i64,
        title: String,
        content: String,
        author_id: i64,
    ) -> Result<Post, DomainError> {
        Ok(self
            .post_repository
            .update(id, title, content, author_id)
            .await?)
    }

    pub async fn delete_post(&self, id: i64, author_id: i64) -> Result<bool, DomainError> {
        Ok(self.post_repository.delete(id, author_id).await?)
    }

    pub async fn get_post(&self, id: i64) -> Result<Post, DomainError> {
        //todo!("Implement get post")
        Ok(self.post_repository.find_by_id(id).await?)
    }

    pub async fn get_posts(&self, offset: i32, limit: i32) -> Result<Vec<Post>, DomainError> {
        //todo!("Implement list posts")
        let posts = self
            .post_repository
            .list(offset as i64, limit as i64)
            .await?;

        println!(
            "🔍 BlogService::get_posts - Service: {:p}",
            self as *const _
        );

        Ok(posts)
    }
}

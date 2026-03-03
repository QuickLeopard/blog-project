use std::sync::Arc;

use crate::data::PostRepository;
use crate::domain::{error::DomainError, post::Post};

const TITLE_MAX_LEN: usize = 200;
const CONTENT_MAX_LEN: usize = 50_000;

pub struct BlogService {
    post_repository: Arc<dyn PostRepository>,
}

impl BlogService {
    pub fn new(post_repository: Arc<dyn PostRepository>) -> Self {
        Self { post_repository }
    }

    fn validate_post(title: &str, content: &str) -> Result<(), DomainError> {
        let title = title.trim();
        let content = content.trim();

        if title.is_empty() {
            return Err(DomainError::ValidationError(
                "Title must not be empty".into(),
            ));
        }
        if title.len() > TITLE_MAX_LEN {
            return Err(DomainError::ValidationError(
                format!("Title must not exceed {} characters", TITLE_MAX_LEN),
            ));
        }
        if content.is_empty() {
            return Err(DomainError::ValidationError(
                "Content must not be empty".into(),
            ));
        }
        if content.len() > CONTENT_MAX_LEN {
            return Err(DomainError::ValidationError(
                format!("Content must not exceed {} characters", CONTENT_MAX_LEN),
            ));
        }
        Ok(())
    }

    pub async fn create_post(
        &self,
        title: String,
        content: String,
        author_id: i64,
    ) -> Result<Post, DomainError> {
        Self::validate_post(&title, &content)?;
        self.post_repository
            .create(title.trim().to_string(), content.trim().to_string(), author_id)
            .await
    }

    pub async fn update_post(
        &self,
        id: i64,
        title: String,
        content: String,
        author_id: i64,
    ) -> Result<Post, DomainError> {
        Self::validate_post(&title, &content)?;
        self.post_repository
            .update(id, title.trim().to_string(), content.trim().to_string(), author_id)
            .await
    }

    pub async fn delete_post(&self, id: i64, author_id: i64) -> Result<bool, DomainError> {
        self.post_repository.delete(id, author_id).await
    }

    pub async fn get_post(&self, id: i64) -> Result<Post, DomainError> {
        //todo!("Implement get post")
        self.post_repository.find_by_id(id).await
    }

    pub async fn get_posts(&self, offset: i32, limit: i32) -> Result<Vec<Post>, DomainError> {
        let posts = self
            .post_repository
            .list(offset as i64, limit as i64)
            .await?;

        Ok(posts)
    }

    pub async fn count_posts(&self) -> Result<i64, DomainError> {
        self.post_repository.count().await
    }
}

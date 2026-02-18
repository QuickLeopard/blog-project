use crate::domain::{error::DomainError, post::Post};

pub struct BlogService;

impl BlogService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_post(&self, title: String, content: String, author_id: i64) -> Result<Post, DomainError> {
        todo!("Implement post creation")
    }

    pub async fn get_post(&self, id: i64) -> Result<Post, DomainError> {
        todo!("Implement get post")
    }

    pub async fn update_post(&self, id: i64, title: String, content: String, user_id: i64) -> Result<Post, DomainError> {
        todo!("Implement post update")
    }

    pub async fn delete_post(&self, id: i64, user_id: i64) -> Result<(), DomainError> {
        todo!("Implement post deletion")
    }

    pub async fn list_posts(&self, page: i32, page_size: i32) -> Result<(Vec<Post>, i32), DomainError> {
        todo!("Implement list posts")
    }
}

use crate::domain::post::Post;

pub struct PostRepository;

impl PostRepository {
    pub fn new() -> Self {
        Self
    }

    pub async fn create(&self, title: String, content: String, author_id: i64) -> Result<Post, sqlx::Error> {
        todo!("Implement post creation")
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<Post>, sqlx::Error> {
        todo!("Implement find by id")
    }

    pub async fn update(&self, id: i64, title: String, content: String) -> Result<Post, sqlx::Error> {
        todo!("Implement post update")
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        todo!("Implement post deletion")
    }

    pub async fn list(&self, offset: i64, limit: i64) -> Result<Vec<Post>, sqlx::Error> {
        todo!("Implement list posts")
    }

    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        todo!("Implement count posts")
    }
}

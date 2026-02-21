use crate::domain::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {   
    async fn create(&self, username: String, email: String, password_hash: String) -> Result<User, sqlx::Error>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error>;
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error>;
}

use async_trait::async_trait;

use crate::domain::error::DomainError;
use crate::domain::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(
        &self,
        username: String,
        email: String,
        password_hash: String,
    ) -> Result<User, DomainError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;
    #[allow(dead_code)]
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, DomainError>;
}

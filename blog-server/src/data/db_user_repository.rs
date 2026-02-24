use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::data::UserRepository;
use crate::domain::error::DomainError;
use crate::domain::user::User;

pub struct DbUserRepository {
    pool: Pool<Postgres>,
}

impl DbUserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for DbUserRepository {
    async fn create(
        &self,
        username: String,
        email: String,
        password_hash: String,
    ) -> Result<User, DomainError> {
        todo!("Implement user creation")
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        todo!("Implement find by username")
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<User>, DomainError> {
        todo!("Implement find by id")
    }
}

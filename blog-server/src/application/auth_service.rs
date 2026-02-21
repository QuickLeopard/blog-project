use std::sync::Arc;

use crate::domain::{error::DomainError, user::User};
use crate::data::UserRepository;

pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
}

impl AuthService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository}
    }

    pub async fn register(&self, username: String, email: String, password: String) -> Result<User, DomainError> {
        let password_hash = password;
        Ok(self.user_repository.create(username, email, password_hash).await?)
    }

    pub async fn login(&self, username: String, password: String) -> Result<(User, String), DomainError> {
        todo!("Implement user login")
    }

    pub fn verify_token(&self, token: &str) -> Result<i64, DomainError> {
        todo!("Implement token verification")
    }
}

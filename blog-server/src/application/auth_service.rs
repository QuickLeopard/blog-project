use std::sync::Arc;

use crate::data::UserRepository;
use crate::domain::{error::DomainError, user::User};
use crate::infrastructure::hash::{hash_password, verify_password};

pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
}

impl AuthService {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn register(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<User, DomainError> {
        let password_hash = hash_password(&password)?;
        Ok(self
            .user_repository
            .create(username, email, password_hash)
            .await?)
    }

    pub async fn login(&self, username: String, password: String) -> Result<User, DomainError> {
        let user = self
            .user_repository
            .find_by_username(&username)
            .await?
            .ok_or_else(|| DomainError::UserNotFound)?; //

        //.ok_or_else(|| DomainError::UserNotFound(format!("User '{}' not found", &username)))?;

        if verify_password(&password, &user.password_hash)? {
            //todo!("Generate JWT token")
            Ok(user)
        } else {
            Err(DomainError::InvalidCredentials)
        }
    }

    pub fn verify_token(&self, token: &str) -> Result<i64, DomainError> {
        todo!("Implement token verification")
    }
}

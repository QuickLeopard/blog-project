use std::sync::Arc;

use tracing::instrument;

use crate::data::UserRepository;
use crate::domain::{error::DomainError, user::User};
use crate::infrastructure::hash::{hash_password, verify_password};
use crate::infrastructure::jwt::JwtService;

pub struct AuthService {
    user_repository: Arc<dyn UserRepository>,
    jwt_service: JwtService
}

impl AuthService {
    pub fn new(user_repository: Arc<dyn UserRepository>, jwt_service: JwtService) -> Self {
        Self { user_repository, jwt_service }
    }

    #[instrument(skip(self))]
    pub async fn register(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<(User, String), DomainError> {
        let password_hash = hash_password(&password)?;
        let user = self
            .user_repository
            .create(username, email, password_hash)
            .await?;

        let token = self.jwt_service.generate_token(user.id, &user.username)?;

        Ok((user, token))
    }

    #[instrument(skip(self))]
    pub async fn login(&self, username: String, password: String) -> Result<(User, String), DomainError> {
        let user = self
            .user_repository
            .find_by_username(&username)
            .await?
            .ok_or_else(|| DomainError::UserNotFound)?; //

        //.ok_or_else(|| DomainError::UserNotFound(format!("User '{}' not found", &username)))?;

        if verify_password(&password, &user.password_hash)? {           
            Ok((user.clone(), self.jwt_service.generate_token(user.id, &user.username)?))
        } else {
            Err(DomainError::InvalidCredentials)
        }
    }

    /*pub fn verify_token(&self, token: &str) -> Result<i64, DomainError> {
        todo!("Implement token verification")
    }*/
}

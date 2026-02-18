use crate::domain::{error::DomainError, user::User};

pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        Self
    }

    pub async fn register(&self, username: String, email: String, password: String) -> Result<User, DomainError> {
        todo!("Implement user registration")
    }

    pub async fn login(&self, username: String, password: String) -> Result<(User, String), DomainError> {
        todo!("Implement user login")
    }

    pub fn verify_token(&self, token: &str) -> Result<i64, DomainError> {
        todo!("Implement token verification")
    }
}

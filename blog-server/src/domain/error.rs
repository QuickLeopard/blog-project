use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("User not found")]
    UserNotFound,
    
    #[error("User already exists: {0}")]
    UserAlreadyExists(String),
    
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Post not found")]
    PostNotFound,
    
    #[error("Forbidden: you don't have permission to perform this action")]
    Forbidden,
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<sqlx::Error> for DomainError {
    fn from(err: sqlx::Error) -> Self {
        DomainError::InternalError(err.to_string())
    }
}


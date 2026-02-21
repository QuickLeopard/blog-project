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

    #[error("Database row not found")]
    DatabaseRowNotFound,
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<sqlx::Error> for DomainError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DomainError::DatabaseRowNotFound,
            _ => DomainError::InternalError(err.to_string()),
        }
    }
}

impl From<DomainError> for tonic::Status {
    fn from(err: DomainError) -> Self {
        
        match err {
            DomainError::UserNotFound => tonic::Status::not_found(err.to_string()),
            DomainError::PostNotFound => tonic::Status::not_found(err.to_string()),
            DomainError::DatabaseRowNotFound => tonic::Status::not_found(err.to_string()),
            DomainError::InvalidCredentials => tonic::Status::unauthenticated(err.to_string()),           
            DomainError::Forbidden => tonic::Status::permission_denied(err.to_string()),
            _ => tonic::Status::internal(err.to_string()),
        }
    }
}


use actix_web::{HttpResponse, ResponseError, http::StatusCode};

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
    DatabaseError(#[from] sqlx::Error),

    #[error("Argon2 error: {0}")]
    Argon2Error(String),

    #[error("JWT error: {0}")]
    JWTError(#[from] jsonwebtoken::errors::Error),

    /*#[error("Database row not found")]
    DatabaseRowNotFound,*/
    #[error("Internal error: {0}")]
    InternalError(String),
}

/*impl From<sqlx::Error> for DomainError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DomainError::DatabaseRowNotFound,
            _ => DomainError::InternalError(err.to_string()),
        }
    }
}*/

impl From<argon2::password_hash::Error> for DomainError {
    fn from(err: argon2::password_hash::Error) -> Self {
        DomainError::Argon2Error(err.to_string())
    }
}

impl ResponseError for DomainError {
    fn error_response(&self) -> HttpResponse {
        let status = match self {
            //DomainError::Validation(_) => StatusCode::BAD_REQUEST,
            DomainError::UserNotFound => StatusCode::NOT_FOUND,
            DomainError::PostNotFound => StatusCode::NOT_FOUND,
            DomainError::UserAlreadyExists(_) => StatusCode::CONFLICT,
            DomainError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            DomainError::Forbidden => StatusCode::FORBIDDEN,
            DomainError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DomainError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": self.to_string(),
            "status": status.as_u16(),
        }))
    }
}

impl From<DomainError> for tonic::Status {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::UserNotFound => tonic::Status::not_found(err.to_string()),
            DomainError::PostNotFound => tonic::Status::not_found(err.to_string()),
            DomainError::DatabaseError(_) => tonic::Status::internal(err.to_string()),
            //DomainError::DatabaseRowNotFound => tonic::Status::not_found(err.to_string()),
            DomainError::InvalidCredentials => tonic::Status::unauthenticated(err.to_string()),
            DomainError::Forbidden => tonic::Status::permission_denied(err.to_string()),
            _ => tonic::Status::internal(err.to_string()),
        }
    }
}

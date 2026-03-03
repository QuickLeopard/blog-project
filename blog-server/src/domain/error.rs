use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use tracing::{error, warn};

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

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Argon2 error: {0}")]
    Argon2Error(String),

    #[error("JWT error: {0}")]
    JWTError(#[from] jsonwebtoken::errors::Error),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl DomainError {
    fn log(&self) {
        match self {
            DomainError::DatabaseError(e) => error!(error = %e, "Database error"),
            DomainError::Argon2Error(e) => error!(error = %e, "Argon2 hashing error"),
            DomainError::InternalError(e) => error!(error = %e, "Internal server error"),
            DomainError::JWTError(e) => warn!(error = %e, "JWT authentication error"),
            DomainError::InvalidCredentials => warn!("Invalid credentials attempt"),
            DomainError::Forbidden => warn!("Forbidden access attempt"),
            DomainError::UserNotFound => warn!("User not found"),
            DomainError::PostNotFound => warn!("Post not found"),
            DomainError::UserAlreadyExists(msg) => warn!(details = %msg, "User already exists"),
            DomainError::ValidationError(msg) => warn!(details = %msg, "Validation error"),
        }
    }
}

impl From<argon2::password_hash::Error> for DomainError {
    fn from(err: argon2::password_hash::Error) -> Self {
        DomainError::Argon2Error(err.to_string())
    }
}

impl ResponseError for DomainError {
    fn error_response(&self) -> HttpResponse {
        self.log();

        let status = match self {
            DomainError::UserNotFound => StatusCode::NOT_FOUND,
            DomainError::PostNotFound => StatusCode::NOT_FOUND,
            DomainError::UserAlreadyExists(_) => StatusCode::CONFLICT,
            DomainError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            DomainError::Forbidden => StatusCode::FORBIDDEN,
            DomainError::ValidationError(_) => StatusCode::BAD_REQUEST,
            DomainError::DatabaseError(_)
            | DomainError::InternalError(_)
            | DomainError::Argon2Error(_)
            | DomainError::JWTError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": self.to_string(),
            "status": status.as_u16(),
        }))
    }
}

impl From<DomainError> for tonic::Status {
    fn from(err: DomainError) -> Self {
        err.log();

        match err {
            DomainError::UserNotFound => tonic::Status::not_found(err.to_string()),
            DomainError::PostNotFound => tonic::Status::not_found(err.to_string()),
            DomainError::DatabaseError(_) => tonic::Status::internal(err.to_string()),
            DomainError::JWTError(_) => tonic::Status::unauthenticated(err.to_string()),
            DomainError::InvalidCredentials => tonic::Status::unauthenticated(err.to_string()),
            DomainError::Forbidden => tonic::Status::permission_denied(err.to_string()),
            DomainError::ValidationError(_) => tonic::Status::invalid_argument(err.to_string()),
            DomainError::UserAlreadyExists(_) => tonic::Status::already_exists(err.to_string()),
            DomainError::Argon2Error(_) | DomainError::InternalError(_) => {
                tonic::Status::internal(err.to_string())
            }
        }
    }
}

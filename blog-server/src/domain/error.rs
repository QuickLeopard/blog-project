use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use tracing::{error, warn};

use thiserror::Error;

/// Unified error type for all layers of the server (domain, application,
/// infrastructure, and presentation).
///
/// Both the HTTP and gRPC presentation layers convert `DomainError` into their
/// respective protocol error types automatically:
/// - [`ResponseError`] maps it to an Actix-Web [`HttpResponse`] with the
///   appropriate HTTP status code and a JSON body `{"error": "...", "status": N}`.
/// - [`From<DomainError> for tonic::Status`] maps it to a gRPC status code.
///
/// Every variant emits a structured `tracing` log entry via [`DomainError::log`].
#[derive(Debug, Error)]
pub enum DomainError {
    /// The requested user does not exist in the database. Maps to HTTP 404 / gRPC NOT_FOUND.
    #[error("User not found")]
    UserNotFound,

    /// A registration attempt was made with a username or email that is already
    /// taken. The inner `String` carries the conflicting field description.
    /// Maps to HTTP 409 / gRPC ALREADY_EXISTS.
    #[error("User already exists: {0}")]
    UserAlreadyExists(String),

    /// The supplied password does not match the stored hash, or the username
    /// does not exist. The two cases are intentionally merged to avoid username
    /// enumeration. Maps to HTTP 401 / gRPC UNAUTHENTICATED.
    #[error("Invalid credentials")]
    InvalidCredentials,

    /// The requested post does not exist in the database. Also returned when a
    /// post exists but the caller is not its author (see ownership checks in
    /// `db_post_repository`). Maps to HTTP 404 / gRPC NOT_FOUND.
    #[error("Post not found")]
    PostNotFound,

    /// The caller is authenticated but is not allowed to perform the requested
    /// operation (e.g. editing another user's post). Maps to HTTP 403 /
    /// gRPC PERMISSION_DENIED.
    #[error("Forbidden: you don't have permission to perform this action")]
    Forbidden,

    /// Input failed business-rule validation (e.g. empty title, content too
    /// long). The inner `String` describes which rule was violated.
    /// Maps to HTTP 400 / gRPC INVALID_ARGUMENT.
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// A SQLx database operation failed. Constructed automatically via
    /// `#[from] sqlx::Error`. Maps to HTTP 500 / gRPC INTERNAL.
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    /// Argon2 password hashing or verification failed. Constructed via the
    /// manual `From<argon2::password_hash::Error>` impl below because the crate
    /// error type does not implement `std::error::Error` directly.
    /// Maps to HTTP 500 / gRPC INTERNAL.
    #[error("Argon2 error: {0}")]
    Argon2Error(String),

    /// JWT encoding or decoding failed (e.g. expired token, bad signature).
    /// Constructed automatically via `#[from] jsonwebtoken::errors::Error`.
    /// Maps to HTTP 401 / gRPC UNAUTHENTICATED when the token is invalid.
    #[error("JWT error: {0}")]
    JWTError(#[from] jsonwebtoken::errors::Error),

    /// Catch-all for unexpected failures that don't fit other variants.
    /// Maps to HTTP 500 / gRPC INTERNAL.
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl DomainError {
    /// Emits a structured `tracing` log entry at the appropriate severity level.
    /// Infrastructure errors (`Database`, `Argon2`, `Internal`) are logged at
    /// `error!` because they indicate server-side failures requiring attention.
    /// Domain/business errors (`InvalidCredentials`, `NotFound`, etc.) are logged
    /// at `warn!` because they represent expected but noteworthy client behaviour.
    /// Called automatically by both `ResponseError` and `From<DomainError> for
    /// tonic::Status` before building the protocol response.
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

/// Maps `DomainError` to an Actix-Web HTTP response.
/// Logs the error first via [`DomainError::log`], then returns a JSON body:
/// `{"error": "<message>", "status": <code>}`.
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

/// Maps `DomainError` to a gRPC `tonic::Status`.
/// Logs the error first via [`DomainError::log`], then selects the appropriate
/// gRPC status code to mirror the HTTP mapping.
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

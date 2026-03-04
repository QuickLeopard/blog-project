use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Core domain entity representing a registered user stored in the database.
///
/// # Security note
/// `password_hash` is marked `#[serde(skip_serializing)]` so it is **never
/// included** when this struct is serialized to JSON (e.g. in API responses).
/// This is a deliberate security measure — the Argon2 hash must never reach
/// the client. For read-only API responses, consider using [`LoginUserResponse`]
/// which does not contain this field at all.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    /// Argon2 password hash. Never serialized to JSON due to
    /// `#[serde(skip_serializing)]`.
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    /// Convenience constructor used in tests and in-memory repository
    /// implementations. Production code receives `User` values deserialized
    /// from SQLx query results via `sqlx::FromRow`.
    #[allow(dead_code)]
    pub fn new(
        id: i64,
        username: String,
        email: String,
        password_hash: String,
        created_at: DateTime<Utc>,
    ) -> User {
        Self {
            id,
            username,
            email,
            password_hash,
            created_at,
        }
    }
}

/// Deserialized from the HTTP request body for `POST /api/auth/register`.
#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Serialized as the JSON body for both `POST /api/auth/register` (201) and
/// `POST /api/auth/login` (200) responses.
///
/// The embedded [`User`] will not contain `password_hash` in the JSON output
/// due to the `skip_serializing` attribute on that field.
#[derive(Debug, Serialize)]
pub struct LoginUserResponse {
    pub token: String,
    pub user: User,
}

/// Deserialized from the HTTP request body for `POST /api/auth/login`.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

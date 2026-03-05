use async_trait::async_trait;
use sqlx::{PgPool, Row, postgres::PgRow};

use tracing::{debug, info};

use crate::data::UserRepository;
use crate::domain::error::DomainError;
use crate::domain::user::User;

pub struct DbUserRepository {
    pool: PgPool,
}

impl DbUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

fn map_row(row: PgRow) -> Result<User, DomainError> {
    //let decode_err = |e: sqlx::Error| DomainError::Internal(format!("row decode error: {}", e));

    Ok(User {
        id: row.try_get("id")?,                       //.map_err(decode_err)?,
        username: row.try_get("username")?,           //.map_err(decode_err)?,
        email: row.try_get("email")?,                 //.map_err(decode_err)?,
        password_hash: row.try_get("password_hash")?, //.map_err(decode_err)?,
        created_at: row.try_get("created_at")?,       //.map_err(decode_err)?,
    })
}

#[async_trait]
impl UserRepository for DbUserRepository {
    async fn create(
        &self,
        username: String,
        email: String,
        password_hash: String,
    ) -> Result<User, DomainError> {
        let row = sqlx::query(
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, username, email, password_hash, created_at
            "#,
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            // Postgres error code 23505 = unique_violation (duplicate username or email).
            // Check the constraint name to produce a specific, user-facing message.
            if let sqlx::Error::Database(ref db_err) = e {
                if db_err.code().as_deref() == Some("23505") {
                    let msg = match db_err.constraint() {
                        Some(c) if c.contains("email") => "Email is already taken",
                        _ => "Username is already taken",
                    };
                    return DomainError::UserAlreadyExists(msg.to_string());
                }
            }
            DomainError::DatabaseError(e)
        })?;

        let user = map_row(row)?;
        info!(user_id = user.id, "user created");
        Ok(user)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, created_at
            FROM users
            WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let user = map_row(row)?;
                info!(user_id = user.id, "user found by username {}", username);
                Ok(Some(user))
            }
            None => {
                debug!("user not found by username {}", username);
                Ok(None)
            }
        }
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<User>, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, username, email, password_hash, created_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let user = map_row(row)?;
                info!(user_id = user.id, "user found by id {}", id);
                Ok(Some(user))
            }
            None => {
                debug!("user not found by id {}", id);
                Ok(None)
            }
        }
    }
}

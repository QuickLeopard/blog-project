use async_trait::async_trait;
use sqlx::{PgPool, Row, postgres::PgRow};

use tracing::{debug, info};

use crate::data::PostRepository;
use crate::domain::error::DomainError;
use crate::domain::post::Post;

pub struct DBPostRepository {
    pool: PgPool,
}

impl DBPostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn check_ownership(&self, post_id: i64, author_id: i64) -> Result<(), DomainError> {
        let row: Option<(i64,)> =
            sqlx::query_as("SELECT author_id FROM posts WHERE id = $1")
                .bind(post_id)
                .fetch_optional(&self.pool)
                .await?;

        match row {
            None => Err(DomainError::PostNotFound),
            Some((owner_id,)) if owner_id != author_id => Err(DomainError::Forbidden),
            _ => Ok(()),
        }
    }
}

fn map_row(row: PgRow) -> Result<Post, DomainError> {
    //let decode_err = |e: sqlx::Error| DomainError::Internal(format!("row decode error: {}", e));

    Ok(Post {
        id: row.try_get("id")?,                 //.map_err(decode_err)?,
        title: row.try_get("title")?,           //.map_err(decode_err)?,
        content: row.try_get("content")?,       //.map_err(decode_err)?,
        author_id: row.try_get("author_id")?,   //.map_err(decode_err)?,
        created_at: row.try_get("created_at")?, //.map_err(decode_err)?,
        updated_at: row.try_get("updated_at")?, //.map_err(decode_err)?,
    })
}

#[async_trait]
impl PostRepository for DBPostRepository {
    async fn create(
        &self,
        title: String,
        content: String,
        author_id: i64,
    ) -> Result<Post, DomainError> {
        let row = sqlx::query(
            r#"
            INSERT INTO posts (title, content, author_id)
            VALUES ($1, $2, $3)
            RETURNING id, title, content, author_id, created_at, updated_at
            "#,
        )
        .bind(title)
        .bind(content)
        .bind(author_id)
        .fetch_one(&self.pool)
        .await?;

        let post = map_row(row)?;
        info!(post_id = post.id, "post created");
        Ok(post)
    }

    async fn find_by_id(&self, id: i64) -> Result<Post, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM posts WHERE id = $1
        "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => DomainError::PostNotFound,
            _ => DomainError::DatabaseError(e),
        })?;

        let post = map_row(row)?;
        debug!(post_id = post.id, "post fetched by id");
        Ok(post)
    }

    async fn update(
        &self,
        id: i64,
        title: String,
        content: String,
        author_id: i64,
    ) -> Result<Post, DomainError> {
        self.check_ownership(id, author_id).await?;

        let row = sqlx::query(
            r#"
            UPDATE posts
            SET title = $1, content = $2, updated_at = NOW()
            WHERE id = $3 AND author_id = $4
            RETURNING id, title, content, author_id, created_at, updated_at
            "#,
        )
        .bind(title)
        .bind(content)
        .bind(id)
        .bind(author_id)
        .fetch_one(&self.pool)
        .await?;

        let post = map_row(row)?;
        info!(post_id = post.id, "post updated");
        Ok(post)
    }

    async fn delete(&self, id: i64, author_id: i64) -> Result<bool, DomainError> {
        self.check_ownership(id, author_id).await?;

        sqlx::query("DELETE FROM posts WHERE id = $1 AND author_id = $2")
            .bind(id)
            .bind(author_id)
            .execute(&self.pool)
            .await?;

        info!(post_id = id, "post deleted");
        Ok(true)
    }

    async fn list(&self, offset: i64, limit: i64) -> Result<Vec<Post>, DomainError> {
        let rows = sqlx::query(
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM posts
            ORDER BY created_at DESC
            OFFSET $1 LIMIT $2
            "#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let posts = rows
            .into_iter()
            .map(map_row)
            .collect::<Result<Vec<_>, _>>()?;
        debug!("fetched {} posts from database", posts.len());
        Ok(posts)
    }

    async fn count(&self) -> Result<i64, DomainError> {
        let rows = sqlx::query("SELECT COUNT(*) FROM posts")
            .fetch_one(&self.pool)
            .await?;
        let count: i64 = rows.try_get(0)?;
        Ok(count)
    }
}

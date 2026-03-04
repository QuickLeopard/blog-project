use sqlx::{PgPool, migrate, postgres::PgPoolOptions};

/// Maximum number of simultaneous database connections in the pool.
const DB_MAX_CONNECTIONS: u32 = 20;

/// Minimum number of connections kept alive in the pool (warm pool).
const DB_MIN_CONNECTIONS: u32 = 5;

/// How long to wait for a free connection before returning an error.
const DB_ACQUIRE_TIMEOUT_SECS: u64 = 5;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(DB_MAX_CONNECTIONS)
        .min_connections(DB_MIN_CONNECTIONS)
        .acquire_timeout(std::time::Duration::from_secs(DB_ACQUIRE_TIMEOUT_SECS))
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Run all migrations from migrations/
    migrate!("./migrations").run(pool).await?;
    Ok(())
}

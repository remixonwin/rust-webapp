use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Error;
use std::env;

// Establishes a connection pool to the database using the DATABASE_URL environment variable
pub async fn create_pool() -> Result<PgPool, Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

// Utility function for test setup and database initialization
#[cfg(test)]
pub async fn create_test_pool() -> Result<PgPool, Error> {
    let database_url = env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set");
    
    PgPool::connect(&database_url).await
}

// Diagnostic function to check database connection health
#[cfg(test)]
pub async fn check_connection(pool: &PgPool) -> Result<(), Error> {
    // Verify database connection by acquiring and releasing a connection
    let conn = pool.acquire().await?;
    drop(conn);
    Ok(())
}

#[cfg(test)]
pub async fn run_migrations(pool: &PgPool) -> Result<(), Error> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| Error::Configuration(Box::new(e)))
}

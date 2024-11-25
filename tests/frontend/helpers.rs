use std::sync::Once;
use tokio::runtime::Runtime;
use sqlx::PgPool;

static INIT: Once = Once::new();

pub async fn setup_test_db() -> PgPool {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
    });

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Clear test data
    sqlx::query!("DELETE FROM auth_logs")
        .execute(&pool)
        .await
        .expect("Failed to clear auth_logs");

    sqlx::query!("DELETE FROM password_reset_tokens")
        .execute(&pool)
        .await
        .expect("Failed to clear password_reset_tokens");

    sqlx::query!("DELETE FROM users")
        .execute(&pool)
        .await
        .expect("Failed to clear users");

    pool
}

pub fn run_test<F>(test: F) -> ()
where
    F: FnOnce(PgPool) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
{
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let pool = setup_test_db().await;
        if let Err(e) = test(pool) {
            panic!("Test failed: {}", e);
        }
    });
}

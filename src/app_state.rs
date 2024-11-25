use sqlx::PgPool;
use std::sync::Arc;
use crate::auth::rate_limiter::RateLimiter;
use crate::auth::repository::UserRepository;

pub struct AppState {
    pub pool: PgPool,
    pub rate_limiter: Arc<RateLimiter>,
    pub user_repository: UserRepository,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: pool.clone(),
            rate_limiter: Arc::new(RateLimiter::new()),
            user_repository: UserRepository::new(pool),
        }
    }
}

pub mod app_state;
pub mod auth;
pub mod db;
pub mod models;
pub mod error;

// Re-exports for testing
pub use app_state::AppState;
pub use auth::handlers::{RegisterRequest, LoginRequest};
pub use auth::repository::UserRepository;
pub use auth::rate_limiter::RateLimiter;
pub use db::create_pool;

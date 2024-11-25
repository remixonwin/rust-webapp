use actix_web::{
    dev::{Service, ServiceResponse, ServiceRequest},
    test,
    App,
    Error,
    web::Data,
};
use rust_webapp::{
    auth::repository::UserRepository,
    auth::rate_limiter::RateLimiter,
    db::create_test_pool,
    AppState,
    auth::handlers::configure,
};
use std::sync::Arc;

pub async fn init_test_service() -> impl Service<ServiceRequest, Response = ServiceResponse, Error = Error> {
    let pool = create_test_pool().await.expect("Failed to create test pool");
    let user_repository = UserRepository::new(pool.clone());
    let rate_limiter = Arc::new(RateLimiter::new());

    let app_state = AppState {
        pool,
        user_repository,
        rate_limiter,
    };

    let app = App::new()
        .app_data(Data::new(app_state))
        .configure(configure);

    test::init_service(app).await
}

use actix_web::{test, App, dev::Service};
use actix_http::Request;

pub async fn init_test_service() -> impl Service<
    Request,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
> {
    let app = App::new()
        .configure(rust_webapp::routes::configure);
    test::init_service(app).await
}
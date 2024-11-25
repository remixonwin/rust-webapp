pub mod test_server;
pub mod test_utils;

use actix_web::{
    test,
    web::Bytes,
    dev::ServiceRequest,
};
use rust_webapp::common::test_server::init_test_service;

pub async fn init_test_service_with_routes() -> impl actix_web::dev::Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse, Error = actix_web::Error> {
    init_test_service().await
}

pub async fn send_test_request(
    app: &impl actix_web::dev::Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse, Error = actix_web::Error>, 
    req: ServiceRequest
) -> actix_web::dev::ServiceResponse {
    test::call_service(app, req).await
}

pub async fn send_test_request_with_body(
    method: actix_web::http::Method, 
    path: &str, 
    body: Bytes
) -> ServiceRequest {
    test_utils::create_test_request_with_body(method, path, body)
}

use actix_web::{http::StatusCode, test};
use rust_webapp::models::Message;

use crate::common::init_test_service;

#[actix_web::test]
async fn test_health_check() {
    let app = init_test_service().await;
    let req = test::TestRequest::get()
        .uri("/health")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body: Message = test::read_body_json(resp).await;
    assert_eq!(body.content, "Service is healthy");
}

#[actix_web::test]
async fn test_health_check_method_not_allowed() {
    let app = init_test_service().await;
    let req = test::TestRequest::post()
        .uri("/health")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
}

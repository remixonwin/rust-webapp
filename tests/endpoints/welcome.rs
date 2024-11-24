use actix_web::{http::StatusCode, test};

use crate::common::init_test_service;

#[actix_web::test]
async fn test_welcome_page() {
    let app = init_test_service().await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    let content = String::from_utf8(body.to_vec()).unwrap();
    assert!(content.contains("Welcome to Rust WebApp"));
}

#[actix_web::test]
async fn test_welcome_page_method_not_allowed() {
    let app = init_test_service().await;
    let req = test::TestRequest::post().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[actix_web::test]
async fn test_welcome_page_error_handling() {
    let app = init_test_service().await;
    let req = test::TestRequest::get().uri("/nonexistent").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

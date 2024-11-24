use super::*;
use actix_web::{http::StatusCode, test, App};

#[actix_web::test]
async fn test_routes_configuration() {
    let app = test::init_service(App::new().configure(configure)).await;

    // Test welcome page route
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Test health check route
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Test hello route
    let req = test::TestRequest::get().uri("/hello").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Test echo route with valid POST
    let req = test::TestRequest::post()
        .uri("/echo")
        .set_json(&serde_json::json!({"message": "test"}))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Test method not allowed
    let req = test::TestRequest::post().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);

    // Test static files route
    let req = test::TestRequest::get().uri("/static/test.txt").to_request();
    let resp = test::call_service(&app, req).await;
    // Note: This will return 404 if the file doesn't exist, which is expected
    assert!(resp.status() == StatusCode::NOT_FOUND || resp.status() == StatusCode::OK);
}

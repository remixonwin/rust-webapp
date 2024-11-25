use actix_web::test;
use crate::common::test_server::init_test_service;

#[actix_web::test]
async fn test_hello_endpoint() {
    let mut app = init_test_service().await;
    
    let req = test::TestRequest::get()
        .uri("/hello")
        .to_request();
    
    let resp = test::call_service(&mut app, req).await;
    
    assert_eq!(resp.status(), 200);
    
    let body = test::read_body(resp).await;
    assert_eq!(body, "Hello, World!");
}

#[actix_web::test]
async fn test_hello_method_not_allowed() {
    let app = init_test_service().await;
    let req = test::TestRequest::post().uri("/hello").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 405);
}

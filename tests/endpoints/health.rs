use actix_web::test;
use crate::common::test_server::init_test_service;

#[actix_web::test]
async fn test_health_endpoint() {
    let mut app = init_test_service().await;
    
    let req = test::TestRequest::get()
        .uri("/health")
        .to_request();
    
    let resp = test::call_service(&mut app, req).await;
    
    assert_eq!(resp.status(), 200);
    
    let body = test::read_body(resp).await;
    assert_eq!(body, "OK");
}

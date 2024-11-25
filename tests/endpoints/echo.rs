use actix_web::{
    test,
    web::Bytes,
};
use crate::common::test_server::init_test_service;

#[actix_web::test]
async fn test_echo_handler() {
    let mut app = init_test_service().await;
    
    let req = test::TestRequest::post()
        .uri("/echo")
        .set_payload(Bytes::from("Hello, World!"))
        .to_request();
    
    let resp = test::call_service(&mut app, req).await;
    
    assert_eq!(resp.status(), 200);
    
    let body = test::read_body(resp).await;
    assert_eq!(body, "Hello, World!");
}

#[actix_web::test]
async fn test_echo_method_not_allowed() {
    let app = init_test_service().await;
    let req = test::TestRequest::get().uri("/echo").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 405);
}

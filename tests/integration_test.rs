use rust_webapp::{hello, echo, health_check, Message};
use actix_web::{test, App, web};

#[actix_web::test]
async fn test_integration_hello() {
    let app = test::init_service(
        App::new()
            .service(hello)
    ).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp: Message = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.content, "Welcome to Quizmo.me!");
}

#[actix_web::test]
async fn test_integration_echo() {
    let app = test::init_service(
        App::new()
            .service(echo)
    ).await;

    let test_message = Message {
        content: String::from("integration test message")
    };

    let req = test::TestRequest::post()
        .uri("/echo")
        .set_json(&test_message)
        .to_request();
    
    let resp: Message = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.content, test_message.content);
}

#[actix_web::test]
async fn test_integration_health() {
    let app = test::init_service(
        App::new()
            .service(health_check)
    ).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp: Message = test::call_and_read_body_json(&app, req).await;
    assert_eq!(resp.content, "Service is healthy");
}

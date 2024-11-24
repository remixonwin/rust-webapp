use actix_web::{http::StatusCode, test};
use rust_webapp::{create_app, Message};

#[actix_web::test]
async fn test_integration_hello_success() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::get().uri("/hello").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: Message = test::read_body_json(resp).await;
    assert_eq!(body.content, "Hello, World!");
}

#[actix_web::test]
async fn test_integration_hello_wrong_method() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::post().uri("/hello").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[actix_web::test]
async fn test_integration_echo_success() {
    let app = test::init_service(create_app()).await;
    let test_message = Message {
        content: String::from("integration test message"),
    };

    let req = test::TestRequest::post()
        .uri("/echo")
        .set_json(&test_message)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Message = test::read_body_json(resp).await;
    assert_eq!(body.content, test_message.content);
}

#[actix_web::test]
async fn test_integration_echo_empty_content() {
    let app = test::init_service(create_app()).await;
    let test_message = Message {
        content: String::new(),
    };

    let req = test::TestRequest::post()
        .uri("/echo")
        .set_json(&test_message)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Message = test::read_body_json(resp).await;
    assert_eq!(body.content, "");
}

#[actix_web::test]
async fn test_integration_echo_long_content() {
    let app = test::init_service(create_app()).await;
    let long_content = "a".repeat(1000);
    let test_message = Message {
        content: long_content.clone(),
    };

    let req = test::TestRequest::post()
        .uri("/echo")
        .set_json(&test_message)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Message = test::read_body_json(resp).await;
    assert_eq!(body.content, long_content);
}

#[actix_web::test]
async fn test_integration_echo_special_characters() {
    let app = test::init_service(create_app()).await;
    let special_content = "!@#$%^&*()_+-=[]{}|;:'\",.<>?/~`";
    let test_message = Message {
        content: special_content.to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/echo")
        .set_json(&test_message)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Message = test::read_body_json(resp).await;
    assert_eq!(body.content, special_content);
}

#[actix_web::test]
async fn test_integration_echo_unicode() {
    let app = test::init_service(create_app()).await;
    let unicode_content = "Hello, 世界! 🌍 привет мир";
    let test_message = Message {
        content: unicode_content.to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/echo")
        .set_json(&test_message)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Message = test::read_body_json(resp).await;
    assert_eq!(body.content, unicode_content);
}

#[actix_web::test]
async fn test_integration_echo_wrong_method() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::get().uri("/echo").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[actix_web::test]
async fn test_integration_health_success() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body: Message = test::read_body_json(resp).await;
    assert_eq!(body.content, "Service is healthy");
}

#[actix_web::test]
async fn test_integration_health_wrong_method() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::post().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[actix_web::test]
async fn test_integration_echo_invalid_json() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::post()
        .uri("/echo")
        .set_payload("invalid json")
        .insert_header(("content-type", "application/json"))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_integration_echo_missing_content_type() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::post()
        .uri("/echo")
        .set_payload(r#"{"content":"test"}"#)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

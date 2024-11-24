use actix_web::{test};
use rust_webapp::create_app;
use actix_web::http::{header, StatusCode};

#[actix_web::test]
async fn test_welcome_page() {
    let app = test::init_service(create_app()).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::OK);
    assert!(
        resp.headers()
            .get(header::CONTENT_TYPE)
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("text/html")
    );
}

#[actix_web::test]
async fn test_static_files() {
    let app = test::init_service(create_app()).await;

    let req = test::TestRequest::get()
        .uri("/static/index.html")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::OK);
    assert!(
        resp.headers()
            .get(header::CONTENT_TYPE)
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("text/html")
    );
}

#[actix_web::test]
async fn test_health_endpoint() {
    let app = test::init_service(create_app()).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_compression_enabled() {
    let app = test::init_service(create_app()).await;

    let req = test::TestRequest::get()
        .uri("/")
        .insert_header((header::ACCEPT_ENCODING, "gzip"))
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::OK);
    assert!(resp.headers().contains_key(header::CONTENT_ENCODING));
}

#[actix_web::test]
async fn test_nonexistent_page() {
    let app = test::init_service(create_app()).await;

    let req = test::TestRequest::get()
        .uri("/nonexistent")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

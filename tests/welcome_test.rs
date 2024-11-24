use actix_web::{
    http::{header, StatusCode},
    test, App,
};
use rust_webapp::config;

#[actix_web::test]
async fn test_welcome_page_success() {
    let app = test::init_service(App::new().configure(config)).await;

    let req = test::TestRequest::get().uri("/").to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_welcome_page_content_type() {
    let app = test::init_service(App::new().configure(config)).await;

    let req = test::TestRequest::get().uri("/").to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.headers().contains_key(header::CONTENT_TYPE));
    assert_eq!(
        resp.headers()
            .get(header::CONTENT_TYPE)
            .unwrap()
            .to_str()
            .unwrap(),
        "text/html"
    );
}

#[actix_web::test]
async fn test_static_files_access() {
    let app = test::init_service(App::new().configure(config)).await;

    let req = test::TestRequest::get()
        .uri("/static/index.html")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

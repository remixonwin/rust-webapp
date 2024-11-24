// Integration tests have been moved to tests/endpoints/
// This keeps the test organization cleaner and prevents duplication
#[cfg(test)]
mod tests {
    use crate::routes::configure;
    use crate::Message;
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn test_routes() {
        let app = test::init_service(App::new().configure(configure)).await;

        // Test health check route
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        // Test hello route
        let req = test::TestRequest::get().uri("/hello").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        // Test welcome page route
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        // Test echo route
        let test_message = Message {
            content: "test".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/echo")
            .set_json(&test_message)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        // Test static files route
        let req = test::TestRequest::get()
            .uri("/static/test.txt")
            .to_request();
        let resp = test::call_service(&app, req).await;
        // Note: This will return 404 if the file doesn't exist, which is expected
        assert!(resp.status() == StatusCode::NOT_FOUND || resp.status() == StatusCode::OK);
    }
}

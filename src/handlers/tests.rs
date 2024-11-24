// Tests have been moved to tests/endpoints/
// This keeps the test organization cleaner and prevents duplication
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::StatusCode,
        test,
        web::{self, Json},
        App,
    };
    use serde_json::json;

    #[actix_web::test]
    async fn test_hello() {
        let app = test::init_service(
            App::new().service(web::resource("/hello").route(web::get().to(hello))),
        )
        .await;

        let req = test::TestRequest::get().uri("/hello").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello, World!");
    }

    #[actix_web::test]
    async fn test_echo() {
        let app = test::init_service(
            App::new().service(web::resource("/echo").route(web::post().to(echo))),
        )
        .await;

        let message = json!({
            "message": "test message"
        });

        let req = test::TestRequest::post()
            .uri("/echo")
            .set_json(&message)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let result: Message = test::read_body_json(resp).await;
        assert_eq!(result.message, "test message");
    }

    #[actix_web::test]
    async fn test_method_not_allowed() {
        let app = test::init_service(App::new().default_service(web::to(method_not_allowed))).await;

        let req = test::TestRequest::post().uri("/invalid").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
}

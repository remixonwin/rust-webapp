use actix_web::HttpServer;
use log::info;
use rust_webapp::create_app;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    info!("Starting server at: {}", addr);

    HttpServer::new(create_app).bind(&addr)?.run().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header, test};
    use rust_webapp::Message;

    #[actix_web::test]
    async fn test_hello_endpoint() {
        let app = test::init_service(create_app()).await;
        let req = test::TestRequest::get().uri("/hello").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body: Message = test::read_body_json(resp).await;
        assert_eq!(body.content, "Hello, World!");
    }

    #[actix_web::test]
    async fn test_hello_content_type() {
        let app = test::init_service(create_app()).await;
        let req = test::TestRequest::get().uri("/hello").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(
            resp.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/json"
        );
    }

    #[actix_web::test]
    async fn test_health_endpoint() {
        let app = test::init_service(create_app()).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body: Message = test::read_body_json(resp).await;
        assert_eq!(body.content, "Service is healthy");
    }

    #[actix_web::test]
    async fn test_health_content_type() {
        let app = test::init_service(create_app()).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(
            resp.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/json"
        );
    }

    #[actix_web::test]
    async fn test_echo_endpoint() {
        let app = test::init_service(create_app()).await;
        let test_message = Message {
            content: String::from("test message"),
        };

        let req = test::TestRequest::post()
            .uri("/echo")
            .set_json(&test_message)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: Message = test::read_body_json(resp).await;
        assert_eq!(body.content, test_message.content);
    }

    #[actix_web::test]
    async fn test_echo_content_type() {
        let app = test::init_service(create_app()).await;
        let test_message = Message {
            content: String::from("test message"),
        };

        let req = test::TestRequest::post()
            .uri("/echo")
            .set_json(&test_message)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/json"
        );
    }

    #[actix_web::test]
    async fn test_unicode_echo() {
        let app = test::init_service(create_app()).await;
        let test_message = Message {
            content: String::from("Hello, 世界! 🌍"),
        };

        let req = test::TestRequest::post()
            .uri("/echo")
            .set_json(&test_message)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: Message = test::read_body_json(resp).await;
        assert_eq!(body.content, "Hello, 世界! 🌍");
    }

    #[actix_web::test]
    async fn test_compression() {
        let app = test::init_service(create_app()).await;
        let req = test::TestRequest::get()
            .insert_header(("Accept-Encoding", "gzip"))
            .uri("/")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        assert!(resp.headers().contains_key(header::CONTENT_ENCODING));
    }
}

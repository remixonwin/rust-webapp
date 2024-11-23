use actix_web::{middleware, App, HttpServer};
use log::info;
use std::env;
use rust_webapp::{hello, echo, health_check};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Get port from environment variable or use default
    let port = env::var("PORT").unwrap_or_else(|_| "80".to_string());
    let port: u16 = port.parse().expect("PORT must be a number");

    info!("Starting server on port {}", port);

    HttpServer::new(|| {
        App::new()
            // Enable logger middleware
            .wrap(middleware::Logger::default())
            // Enable compression
            .wrap(middleware::Compress::default())
            // Services
            .service(hello)
            .service(echo)
            .service(health_check)
    })
    .bind(("0.0.0.0", port))?
    .workers(2) // Number of worker threads
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use rust_webapp::Message;

    #[actix_web::test]
    async fn test_hello_endpoint() {
        let app = test::init_service(
            App::new()
                .service(hello)
        ).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(
            App::new()
                .service(health_check)
        ).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_echo_endpoint() {
        let app = test::init_service(
            App::new()
                .service(echo)
        ).await;

        let test_message = Message {
            content: String::from("test message")
        };

        let req = test::TestRequest::post()
            .uri("/echo")
            .set_json(&test_message)
            .to_request();
        
        let resp: Message = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.content, test_message.content);
    }
}

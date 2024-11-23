use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Message {
        content: String::from("Welcome to Quizmo.me!"),
    })
}

#[post("/echo")]
async fn echo(msg: web::Json<Message>) -> impl Responder {
    HttpResponse::Ok().json(Message {
        content: msg.content.clone(),
    })
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(Message {
        content: String::from("Service is healthy"),
    })
}

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

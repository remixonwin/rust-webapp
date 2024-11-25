use actix_web::{web, App, HttpServer, HttpResponse, get};
use actix_cors::Cors;
use dotenv::dotenv;
use actix_web::middleware::Logger;
use serde_json::json;

mod auth;
mod db;
mod models;
mod error;

pub use crate::app_state::AppState;
mod app_state;

// Hello endpoint for testing
#[get("/hello")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "message": "Hello from Rust backend!"
    }))
}

// Default handler to return 404 for unmatched routes
async fn default_handler() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("application/json")
        .json(serde_json::json!({
            "error": "Not Found",
            "message": "The requested endpoint does not exist"
        }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");

    let state = web::Data::new(AppState::new(pool));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        println!("Configuring server with CORS and routes");

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(state.clone())
            .service(
                web::scope("/api")
                    .configure(auth::handlers::configure)
                    .service(hello)
            )
            .default_service(web::route().to(default_handler))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

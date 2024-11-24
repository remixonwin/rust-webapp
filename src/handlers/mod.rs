mod echo;
mod error;
mod health;
mod hello;
mod tests;
mod welcome;

pub use echo::echo;
pub use error::{method_not_allowed, not_found};
pub use health::health_check;
pub use hello::hello;
pub use welcome::welcome_page;

use actix_web::{web, HttpResponse, Responder};
use crate::models::Message;

#[cfg(test)]
mod tests;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn welcome_page() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Rust WebApp!")
}

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

pub async fn echo(message: web::Json<Message>) -> impl Responder {
    HttpResponse::Ok().json(message.0)
}

pub async fn method_not_allowed() -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

use actix_web::{get, post, web, HttpResponse, Responder, error::JsonPayloadError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Message {
    pub content: String,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Message {
        content: String::from("Welcome to Quizmo.me!"),
    })
}

#[post("/echo")]
pub async fn echo(msg: web::Json<Message>) -> impl Responder {
    HttpResponse::Ok().json(Message {
        content: msg.content.clone(),
    })
}

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(Message {
        content: String::from("Service is healthy"),
    })
}

async fn method_not_allowed() -> impl Responder {
    HttpResponse::MethodNotAllowed().finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::JsonConfig::default().error_handler(|err, _| {
        match err {
            JsonPayloadError::ContentType => {
                actix_web::error::ErrorUnsupportedMediaType("Unsupported Media Type")
            }
            _ => actix_web::error::ErrorBadRequest("Invalid JSON format")
        }
    }))
    .service(hello)
    .service(echo)
    .service(health_check)
    .default_service(web::route().to(method_not_allowed));
}

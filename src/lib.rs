use actix_web::{get, post, web, HttpResponse, Responder};
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

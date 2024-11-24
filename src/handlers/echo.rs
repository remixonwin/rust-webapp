use crate::models::Message;
use actix_web::{web, HttpResponse};

pub async fn echo(msg: web::Json<Message>) -> HttpResponse {
    HttpResponse::Ok().json(Message {
        content: msg.content.clone(),
    })
}

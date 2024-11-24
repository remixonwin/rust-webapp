use actix_web::{web, HttpResponse};
use crate::models::Message;

pub async fn echo(msg: web::Json<Message>) -> HttpResponse {
    HttpResponse::Ok().json(Message {
        content: msg.content.clone(),
    })
}

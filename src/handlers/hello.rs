use actix_web::HttpResponse;
use crate::models::Message;

pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(Message {
        content: String::from("Hello, World!"),
    })
}

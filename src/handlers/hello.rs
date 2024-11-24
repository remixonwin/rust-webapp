use crate::models::Message;
use actix_web::HttpResponse;

pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(Message {
        content: String::from("Hello, World!"),
    })
}

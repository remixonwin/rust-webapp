use actix_web::HttpResponse;
use crate::models::Message;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(Message {
        content: String::from("Service is healthy"),
    })
}

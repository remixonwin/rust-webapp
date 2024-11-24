use crate::models::Message;
use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(Message {
        content: String::from("Service is healthy"),
    })
}

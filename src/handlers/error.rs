use actix_web::HttpResponse;

pub async fn method_not_allowed() -> HttpResponse {
    HttpResponse::MethodNotAllowed().finish()
}

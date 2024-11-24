use actix_web::HttpResponse;

pub async fn welcome_page() -> HttpResponse {
    let content = include_str!("../../static/index.html");
    HttpResponse::Ok().content_type("text/html").body(content)
}

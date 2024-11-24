use actix_files as fs;
use actix_http::encoding::Encoder;
use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{http::header, middleware, web, App, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoRequest {
    pub message: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(welcome_page))
            .default_service(web::to(method_not_allowed)),
    )
    .service(
        web::resource("/health")
            .route(web::get().to(health_check))
            .default_service(web::to(method_not_allowed)),
    )
    .service(
        web::resource("/hello")
            .route(web::get().to(hello))
            .default_service(web::to(method_not_allowed)),
    )
    .service(
        web::resource("/echo")
            .route(web::post().to(echo))
            .default_service(web::to(method_not_allowed)),
    )
    .service(fs::Files::new("/static", "./static").show_files_listing())
    .default_service(web::to(not_found));
}

pub fn create_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<EitherBody<Encoder<BoxBody>>>,
        Error = Error,
        InitError = (),
    >,
> {
    App::new()
        .wrap(middleware::Compress::default())
        .configure(config)
}

async fn welcome_page() -> Result<HttpResponse, Error> {
    let content = std::fs::read_to_string("./static/index.html")?;
    Ok(HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "text/html"))
        .body(content))
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(Message {
        content: String::from("Service is healthy"),
    })
}

async fn hello() -> HttpResponse {
    HttpResponse::Ok().json(Message {
        content: String::from("Hello, World!"),
    })
}

async fn echo(msg: web::Json<Message>) -> HttpResponse {
    HttpResponse::Ok().json(Message {
        content: msg.content.clone(),
    })
}

async fn not_found() -> HttpResponse {
    HttpResponse::NotFound().finish()
}

async fn method_not_allowed() -> HttpResponse {
    HttpResponse::MethodNotAllowed()
        .insert_header(("Allow", "GET"))
        .finish()
}

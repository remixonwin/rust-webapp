use crate::handlers;
use actix_files as fs;
use actix_web::web;
use std::path::PathBuf;

#[cfg(test)]
mod tests;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let static_path = {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("static");
        path
    };

    cfg.service(
        web::resource("/")
            .route(web::get().to(handlers::welcome_page))
            .default_service(web::to(handlers::method_not_allowed)),
    )
    .service(
        web::resource("/health")
            .route(web::get().to(handlers::health_check))
            .default_service(web::to(handlers::method_not_allowed)),
    )
    .service(
        web::resource("/hello")
            .route(web::get().to(handlers::hello))
            .default_service(web::to(handlers::method_not_allowed)),
    )
    .service(
        web::resource("/echo")
            .route(web::post().to(handlers::echo))
            .default_service(web::to(handlers::method_not_allowed)),
    )
    .service(fs::Files::new("/static", static_path).show_files_listing());
}

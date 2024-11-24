use actix_web::{Error, HttpResponse, http::header};
use std::path::PathBuf;

pub async fn welcome_page() -> Result<HttpResponse, Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("static");
    path.push("index.html");
    
    let content = std::fs::read_to_string(path)?;
    Ok(HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "text/html"))
        .body(content))
}

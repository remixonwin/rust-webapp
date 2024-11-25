pub mod registration_test;
pub mod login_test;

use actix_web::http::StatusCode;
use crate::common::test_utils::{register_user, login_user};
use crate::common::test_server::init_test_server;

#[actix_web::test]
async fn test_register_success() {
    let app = init_test_server().await;
    let resp = register_user(&app, "test@example.com", "Password123!").await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_register_invalid_email() {
    let app = init_test_server().await;
    let resp = register_user(&app, "invalid-email", "Password123!").await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_register_weak_password() {
    let app = init_test_server().await;
    let resp = register_user(&app, "test@example.com", "weak").await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn test_login_success() {
    let app = init_test_server().await;
    let email = "test@example.com";
    let password = "Password123!";

    // Register first
    let register_resp = register_user(&app, email, password).await;
    assert_eq!(register_resp.status(), StatusCode::OK);

    // Then login
    let login_resp = login_user(&app, email, password).await;
    assert_eq!(login_resp.status(), StatusCode::OK);
}

#[actix_web::test]
async fn test_login_invalid_credentials() {
    let app = init_test_server().await;
    let resp = login_user(&app, "wrong@example.com", "WrongPass123!").await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_web::test]
async fn test_register_duplicate_email() {
    let app = init_test_server().await;
    let email = "test@example.com";
    let password = "Password123!";

    // First registration
    let first_resp = register_user(&app, email, password).await;
    assert_eq!(first_resp.status(), StatusCode::OK);

    // Second registration with same email
    let second_resp = register_user(&app, email, password).await;
    assert_eq!(second_resp.status(), StatusCode::BAD_REQUEST);
}

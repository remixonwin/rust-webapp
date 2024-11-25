use actix_web::{
    test,
    dev::ServiceResponse,
};
use serde_json;

mod common;
use common::{
    register_user, 
    login_user, 
    init_test_service as init_test_app
};

#[actix_rt::test]
async fn test_register_success() {
    let mut app = init_test_app().await;

    let email = "test_register@example.com";
    let password = "StrongPassword123!";

    let register_resp = register_user(&mut app, email, password).await
        .expect("Failed to register user");

    assert_eq!(register_resp.status(), 201);

    let body: serde_json::Value = test::read_body_json(register_resp).await;
    assert!(body.get("user_id").is_some());
}

#[actix_rt::test]
async fn test_register_invalid_email() {
    let mut app = init_test_app().await;

    let email = "invalid_email";
    let password = "StrongPassword123!";

    let register_resp = register_user(&mut app, email, password).await
        .expect("Failed to register user");

    assert_eq!(register_resp.status(), 400);

    let body: serde_json::Value = test::read_body_json(register_resp).await;
    assert!(body.get("error").is_some());
}

#[actix_rt::test]
async fn test_register_duplicate_email() {
    let mut app = init_test_app().await;

    let email = "duplicate_email@example.com";
    let password = "StrongPassword123!";

    let first_register_resp = register_user(&mut app, email, password).await
        .expect("Failed to register user first time");

    assert_eq!(first_register_resp.status(), 201);

    let second_register_resp = register_user(&mut app, email, password).await
        .expect("Failed to attempt duplicate registration");

    assert_eq!(second_register_resp.status(), 409);

    let body: serde_json::Value = test::read_body_json(second_register_resp).await;
    assert!(body.get("error").is_some());
}

#[actix_rt::test]
async fn test_login_success() {
    let mut app = init_test_app().await;

    let email = "test_login@example.com";
    let password = "StrongPassword123!";

    let register_resp = register_user(&mut app, email, password).await
        .expect("Failed to register user");

    assert_eq!(register_resp.status(), 201);

    let login_resp = login_user(&mut app, email, password).await
        .expect("Failed to login user");

    assert_eq!(login_resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(login_resp).await;
    assert!(body.get("token").is_some());
}

#[actix_rt::test]
async fn test_login_invalid_credentials() {
    let mut app = init_test_app().await;

    let email = "test_login_invalid@example.com";
    let password = "StrongPassword123!";

    let register_resp = register_user(&mut app, email, password).await
        .expect("Failed to register user");

    assert_eq!(register_resp.status(), 201);

    let login_resp = login_user(&mut app, email, "WrongPassword123!").await
        .expect("Failed to attempt login");

    assert_eq!(login_resp.status(), 401);

    let body: serde_json::Value = test::read_body_json(login_resp).await;
    assert!(body.get("error").is_some());
}

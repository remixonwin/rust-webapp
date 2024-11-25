use actix_web::{test, http::Method};
use serde_json::json;
use crate::common::test_server::{init_test_app, create_test_request};

#[cfg(test)]
mod login_tests {
    use super::*;
    use uuid::Uuid;

    #[actix_web::test]
    async fn test_empty_login() {
        let app = init_test_app().await;
        let req = create_test_request(Method::POST, "/login")
            .set_json(json!({}))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 400);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["error"].as_str().unwrap().contains("All fields are required"));
    }

    #[actix_web::test]
    async fn test_invalid_email_login() {
        let app = init_test_app().await;
        let req = create_test_request(Method::POST, "/login")
            .set_json(json!({
                "email": "invalid-email",
                "password": "password123"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 400);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["error"].as_str().unwrap().contains("Invalid email format"));
    }

    #[actix_web::test]
    async fn test_nonexistent_user_login() {
        let app = init_test_app().await;
        let req = create_test_request(Method::POST, "/login")
            .set_json(json!({
                "email": "nonexistent@example.com",
                "password": "password123"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 401);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["error"].as_str().unwrap().contains("Invalid credentials"));
    }

    #[actix_web::test]
    async fn test_wrong_password() {
        let app = init_test_app().await;
        let test_email = format!("test{}@example.com", Uuid::new_v4());
        
        // First register
        let register_req = create_test_request(Method::POST, "/register")
            .set_json(json!({
                "email": test_email,
                "password": "correctpassword"
            }))
            .to_request();

        let register_resp = test::call_service(&app, register_req).await;
        assert_eq!(register_resp.status().as_u16(), 200);

        // Try login with wrong password
        let login_req = create_test_request(Method::POST, "/login")
            .set_json(json!({
                "email": test_email,
                "password": "wrongpassword"
            }))
            .to_request();

        let login_resp = test::call_service(&app, login_req).await;
        assert_eq!(login_resp.status().as_u16(), 401);

        let body: serde_json::Value = test::read_body_json(login_resp).await;
        assert!(body["error"].as_str().unwrap().contains("Invalid credentials"));
    }

    #[actix_web::test]
    async fn test_successful_login() {
        let app = init_test_app().await;
        let test_email = format!("test{}@example.com", Uuid::new_v4());
        let test_password = "password123";

        // First register
        let register_req = create_test_request(Method::POST, "/register")
            .set_json(json!({
                "email": test_email,
                "password": test_password
            }))
            .to_request();

        let register_resp = test::call_service(&app, register_req).await;
        assert_eq!(register_resp.status().as_u16(), 200);

        // Then login
        let login_req = create_test_request(Method::POST, "/login")
            .set_json(json!({
                "email": test_email,
                "password": test_password
            }))
            .to_request();

        let login_resp = test::call_service(&app, login_req).await;
        assert_eq!(login_resp.status().as_u16(), 200);

        let body: serde_json::Value = test::read_body_json(login_resp).await;
        assert!(body["message"].as_str().unwrap().contains("Login successful"));
        assert!(body["token"].as_str().is_some());
    }
}

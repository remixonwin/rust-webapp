use actix_web::{test, http::Method};
use serde_json::json;
use crate::common::test_server::{init_test_app, create_test_request};

#[cfg(test)]
mod registration_tests {
    use super::*;
    use uuid::Uuid;

    #[actix_web::test]
    async fn test_empty_registration() {
        let app = init_test_app().await;
        let req = create_test_request(Method::POST, "/register")
            .set_json(json!({}))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 400);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["error"].as_str().unwrap().contains("All fields are required"));
    }

    #[actix_web::test]
    async fn test_invalid_email_registration() {
        let app = init_test_app().await;
        let req = create_test_request(Method::POST, "/register")
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
    async fn test_weak_password() {
        let app = init_test_app().await;
        let req = create_test_request(Method::POST, "/register")
            .set_json(json!({
                "email": "test@example.com",
                "password": "123"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 400);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["error"].as_str().unwrap().contains("Password must be at least 8 characters"));
    }

    #[actix_web::test]
    async fn test_duplicate_email() {
        let app = init_test_app().await;
        let test_email = format!("test{}@example.com", Uuid::new_v4());
        
        // First registration
        let first_req = create_test_request(Method::POST, "/register")
            .set_json(json!({
                "email": test_email,
                "password": "password123"
            }))
            .to_request();

        let first_resp = test::call_service(&app, first_req).await;
        assert_eq!(first_resp.status().as_u16(), 200);

        // Try to register with same email
        let second_req = create_test_request(Method::POST, "/register")
            .set_json(json!({
                "email": test_email,
                "password": "differentpassword123"
            }))
            .to_request();

        let second_resp = test::call_service(&app, second_req).await;
        assert_eq!(second_resp.status().as_u16(), 400);

        let body: serde_json::Value = test::read_body_json(second_resp).await;
        assert!(body["error"].as_str().unwrap().contains("Email already registered"));
    }

    #[actix_web::test]
    async fn test_successful_registration() {
        let app = init_test_app().await;
        let test_email = format!("test{}@example.com", Uuid::new_v4());
        
        let req = create_test_request(Method::POST, "/register")
            .set_json(json!({
                "email": test_email,
                "password": "password123"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 200);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["message"].as_str().unwrap().contains("Registration successful"));
        assert!(body["token"].as_str().is_some());
    }
}

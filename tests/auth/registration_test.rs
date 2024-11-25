use fantoccini::{Client, Locator};
use std::time::Duration;
use tokio::time::sleep;

use crate::common::test_utils::{
    setup_webdriver, 
    navigate_to_url, 
    APP_URL, 
    input_text, 
    click_element,
    get_error_message,
    get_success_message,
    css_locator,
};

async fn register_user(
    client: &Client, 
    email: &str, 
    password: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // Navigate to registration page
    navigate_to_url(client, &format!("{}/register", APP_URL)).await;

    // Find and input email
    input_text(
        client, 
        css_locator("input[name='email']"), 
        email
    ).await;

    // Find and input password
    input_text(
        client, 
        css_locator("input[name='password']"), 
        password
    ).await;

    // Click submit button
    click_element(
        client, 
        css_locator("button[type='submit']")
    ).await;

    // Wait for page to process
    sleep(Duration::from_secs(1)).await;

    Ok(())
}

#[tokio::test]
async fn test_registration_success() {
    let c = setup_webdriver().await;
    
    // Generate a unique email for this test
    let email = format!("test_success_{}@example.com", chrono::Utc::now().timestamp());
    let password = "StrongPassword123!";

    register_user(&c, &email, password).await.expect("Registration failed");

    // Check for success message
    let success = get_success_message(&c)
        .await
        .expect("Success message should be present");
    
    assert!(success.contains("Registration successful"), "Unexpected success message");

    c.close().await.ok();
}

#[tokio::test]
async fn test_registration_duplicate_email() {
    let c = setup_webdriver().await;
    
    // Use a predefined email that should already exist
    let email = "existing@example.com";
    let password = "AnyPassword123!";

    register_user(&c, email, password).await.expect("Registration attempt failed");

    // Check for error message
    let error = get_error_message(&c)
        .await
        .expect("Error message should be present");
    
    assert!(
        error.contains("Email already registered") || 
        error.contains("Duplicate email"),
        "Unexpected error message: {}",
        error
    );

    c.close().await.ok();
}

#[tokio::test]
async fn test_registration_invalid_email() {
    let c = setup_webdriver().await;
    
    // Invalid email format
    let email = "invalid_email";
    let password = "AnyPassword123!";

    register_user(&c, email, password).await.expect("Registration attempt failed");

    // Check for error message
    let error = get_error_message(&c)
        .await
        .expect("Error message should be present");
    
    assert!(
        error.contains("Invalid email") || 
        error.contains("Email format"),
        "Unexpected error message: {}",
        error
    );

    c.close().await.ok();
}

#[tokio::test]
async fn test_registration_weak_password() {
    let c = setup_webdriver().await;
    
    // Generate a unique email for this test
    let email = format!("test_weak_{}@example.com", chrono::Utc::now().timestamp());
    let password = "weak";  // Too short/simple password

    register_user(&c, &email, password).await.expect("Registration attempt failed");

    // Check for error message
    let error = get_error_message(&c)
        .await
        .expect("Error message should be present");
    
    assert!(
        error.contains("Password too weak") || 
        error.contains("Insufficient password complexity"),
        "Unexpected error message: {}",
        error
    );

    c.close().await.ok();
}

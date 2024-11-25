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

async fn login_user(
    client: &Client, 
    email: &str, 
    password: &str
) -> Result<(), Box<dyn std::error::Error>> {
    // Navigate to login page
    navigate_to_url(client, &format!("{}/login", APP_URL)).await;

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
async fn test_login_success() {
    let c = setup_webdriver().await;
    
    // Use a predefined test user
    let email = "test@example.com";
    let password = "TestPassword123!";

    login_user(&c, email, password).await.expect("Login failed");

    // Check for success message or dashboard redirect
    let success = get_success_message(&c)
        .await
        .expect("Success message should be present");
    
    assert!(
        success.contains("Login successful") || 
        success.contains("Welcome"),
        "Unexpected success message"
    );

    c.close().await.ok();
}

#[tokio::test]
async fn test_login_nonexistent_user() {
    let c = setup_webdriver().await;
    
    // Use a nonexistent email
    let email = "nonexistent@example.com";
    let password = "AnyPassword123!";

    login_user(&c, email, password).await.expect("Login attempt failed");

    // Check for error message
    let error = get_error_message(&c)
        .await
        .expect("Error message should be present");
    
    assert!(
        error.contains("User not found") || 
        error.contains("Invalid credentials"),
        "Unexpected error message: {}",
        error
    );

    c.close().await.ok();
}

#[tokio::test]
async fn test_login_incorrect_password() {
    let c = setup_webdriver().await;
    
    // Use an existing email with incorrect password
    let email = "test@example.com";
    let password = "WrongPassword123!";

    login_user(&c, email, password).await.expect("Login attempt failed");

    // Check for error message
    let error = get_error_message(&c)
        .await
        .expect("Error message should be present");
    
    assert!(
        error.contains("Incorrect password") || 
        error.contains("Invalid credentials"),
        "Unexpected error message: {}",
        error
    );

    c.close().await.ok();
}

#[tokio::test]
async fn test_login_empty_fields() {
    let c = setup_webdriver().await;
    
    // Navigate to login page
    navigate_to_url(&c, &format!("{}/login", APP_URL)).await;

    // Click submit button without filling fields
    click_element(
        &c, 
        css_locator("button[type='submit']")
    ).await;

    // Check for error message
    let error = get_error_message(&c)
        .await
        .expect("Error message should be present");
    
    assert!(
        error.contains("Email is required") || 
        error.contains("Password is required"),
        "Unexpected error message: {}",
        error
    );

    c.close().await.ok();
}

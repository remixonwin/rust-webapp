use thirtyfour::prelude::*;
use std::time::Duration;
use serial_test::serial;

mod config;
mod pages;

use config::TEST_CONFIG;
use pages::{
    login_page::LoginPage,
    register_page::RegisterPage,
    dashboard_page::DashboardPage,
};

async fn setup_driver() -> WebDriverResult<WebDriver> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new(&TEST_CONFIG.webdriver_url, caps).await?;
    
    // Set timeouts
    driver.set_implicit_wait_timeout(Duration::from_secs(TEST_CONFIG.implicit_wait_timeout)).await?;
    driver.set_page_load_timeout(Duration::from_secs(TEST_CONFIG.page_load_timeout)).await?;
    driver.set_script_timeout(Duration::from_secs(TEST_CONFIG.script_timeout)).await?;
    
    Ok(driver)
}

#[tokio::test]
#[serial]
async fn test_registration_flow() -> WebDriverResult<()> {
    let driver = setup_driver().await?;

    // Navigate to registration page
    RegisterPage::navigate(&driver).await?;

    // Test invalid email
    RegisterPage::register(&driver, "invalid-email", "Password123!").await?;
    assert_eq!(RegisterPage::get_error_message(&driver).await?, "Invalid email format");

    // Test weak password
    RegisterPage::clear_inputs(&driver).await?;
    RegisterPage::register(&driver, "test@example.com", "weak").await?;
    assert!(RegisterPage::get_error_message(&driver).await?.contains("Password must be at least 8 characters"));

    // Test successful registration
    RegisterPage::clear_inputs(&driver).await?;
    RegisterPage::register(&driver, "test@example.com", "StrongP@ss123").await?;
    assert_eq!(RegisterPage::get_success_message(&driver).await?, "Registration successful");

    driver.quit().await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_login_flow() -> WebDriverResult<()> {
    let driver = setup_driver().await?;

    // Navigate to login page
    LoginPage::navigate(&driver).await?;

    // Test invalid credentials
    LoginPage::login(&driver, "wrong@example.com", "WrongP@ss123").await?;
    assert_eq!(LoginPage::get_error_message(&driver).await?, "Invalid credentials");

    // Test successful login
    LoginPage::clear_inputs(&driver).await?;
    LoginPage::login(&driver, "test@example.com", "StrongP@ss123").await?;
    assert_eq!(LoginPage::get_success_message(&driver).await?, "Login successful");

    // Verify dashboard access
    assert!(DashboardPage::is_welcome_message_visible(&driver).await?);

    driver.quit().await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_account_locking() -> WebDriverResult<()> {
    let driver = setup_driver().await?;

    // Navigate to login page
    LoginPage::navigate(&driver).await?;

    // Attempt multiple failed logins
    for _ in 0..5 {
        LoginPage::login(&driver, "test@example.com", "WrongP@ss123").await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    // Verify account is locked
    LoginPage::clear_inputs(&driver).await?;
    LoginPage::login(&driver, "test@example.com", "StrongP@ss123").await?;
    assert!(LoginPage::get_error_message(&driver).await?.contains("Account is temporarily locked"));

    driver.quit().await?;
    Ok(())
}

#[tokio::test]
#[serial]
async fn test_logout_flow() -> WebDriverResult<()> {
    let driver = setup_driver().await?;

    // Login first
    LoginPage::navigate(&driver).await?;
    LoginPage::login(&driver, "test@example.com", "StrongP@ss123").await?;
    
    // Verify dashboard access
    assert!(DashboardPage::is_welcome_message_visible(&driver).await?);

    // Logout
    DashboardPage::logout(&driver).await?;
    
    // Try to access dashboard (should redirect to login)
    DashboardPage::navigate(&driver).await?;
    assert_eq!(driver.current_url().await?, LoginPage::get_url());

    driver.quit().await?;
    Ok(())
}

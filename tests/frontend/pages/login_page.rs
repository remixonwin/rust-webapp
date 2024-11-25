use thirtyfour::prelude::*;
use super::Page;
use crate::frontend::config::TEST_CONFIG;

pub struct LoginPage;

impl Page for LoginPage {
    fn get_url() -> String {
        format!("{}/login", TEST_CONFIG.app_url)
    }
}

impl LoginPage {
    pub async fn navigate(driver: &WebDriver) -> WebDriverResult<()> {
        driver.get(Self::get_url()).await?;
        Self::wait_for_url(driver).await
    }

    pub async fn login(
        driver: &WebDriver,
        email: &str,
        password: &str,
    ) -> WebDriverResult<()> {
        let email_input = Self::wait_for_element(driver, By::Id("email")).await?;
        let password_input = Self::wait_for_element(driver, By::Id("password")).await?;
        let login_button = Self::wait_for_element_clickable(driver, By::Id("login-button")).await?;

        email_input.send_keys(email).await?;
        password_input.send_keys(password).await?;
        login_button.click().await?;

        Ok(())
    }

    pub async fn get_error_message(driver: &WebDriver) -> WebDriverResult<String> {
        let error_element = Self::wait_for_element(driver, By::ClassName("error-message")).await?;
        error_element.text().await
    }

    pub async fn get_success_message(driver: &WebDriver) -> WebDriverResult<String> {
        let success_element = Self::wait_for_element(driver, By::ClassName("success-message")).await?;
        success_element.text().await
    }

    pub async fn is_error_visible(driver: &WebDriver) -> WebDriverResult<bool> {
        match driver.find_element(By::ClassName("error-message")).await {
            Ok(element) => element.is_displayed().await,
            Err(_) => Ok(false),
        }
    }

    pub async fn clear_inputs(driver: &WebDriver) -> WebDriverResult<()> {
        let email_input = Self::wait_for_element(driver, By::Id("email")).await?;
        let password_input = Self::wait_for_element(driver, By::Id("password")).await?;

        email_input.clear().await?;
        password_input.clear().await?;

        Ok(())
    }
}

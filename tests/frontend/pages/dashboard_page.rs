use thirtyfour::prelude::*;
use super::Page;
use crate::frontend::config::TEST_CONFIG;

pub struct DashboardPage;

impl Page for DashboardPage {
    fn get_url() -> String {
        format!("{}/dashboard", TEST_CONFIG.app_url)
    }
}

impl DashboardPage {
    pub async fn navigate(driver: &WebDriver) -> WebDriverResult<()> {
        driver.get(Self::get_url()).await?;
        Self::wait_for_url(driver).await
    }

    pub async fn logout(driver: &WebDriver) -> WebDriverResult<()> {
        let logout_button = Self::wait_for_element_clickable(driver, By::Id("logout-button")).await?;
        logout_button.click().await?;
        Ok(())
    }

    pub async fn is_welcome_message_visible(driver: &WebDriver) -> WebDriverResult<bool> {
        match driver.find_element(By::XPath("//h2[contains(text(), 'Welcome')]")).await {
            Ok(element) => element.is_displayed().await,
            Err(_) => Ok(false),
        }
    }

    pub async fn get_welcome_message(driver: &WebDriver) -> WebDriverResult<String> {
        let welcome_element = Self::wait_for_element(driver, By::XPath("//h2[contains(text(), 'Welcome')]")).await?;
        welcome_element.text().await
    }
}

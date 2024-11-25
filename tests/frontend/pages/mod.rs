pub mod login_page;
pub mod register_page;
pub mod dashboard_page;

use thirtyfour::prelude::*;
use std::time::Duration;
use crate::frontend::config::TEST_CONFIG;

pub trait Page {
    fn get_url() -> String;
    
    async fn wait_for_url(driver: &WebDriver) -> WebDriverResult<()> {
        let expected_url = Self::get_url();
        let mut retries = 0;
        while driver.current_url().await? != expected_url && retries < 5 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            retries += 1;
        }
        assert_eq!(driver.current_url().await?, expected_url);
        Ok(())
    }

    async fn wait_for_element(driver: &WebDriver, by: By) -> WebDriverResult<WebElement> {
        driver.find_element(by).await
    }

    async fn wait_for_element_clickable(driver: &WebDriver, by: By) -> WebDriverResult<WebElement> {
        let element = Self::wait_for_element(driver, by).await?;
        element.wait_until().clickable().await?;
        Ok(element)
    }

    async fn wait_for_text_present(element: &WebElement, text: &str) -> WebDriverResult<bool> {
        let mut retries = 0;
        while element.text().await? != text && retries < 5 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            retries += 1;
        }
        Ok(element.text().await? == text)
    }
}

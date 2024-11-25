use std::env;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TEST_CONFIG: TestConfig = TestConfig::new();
}

pub struct TestConfig {
    pub app_url: String,
    pub webdriver_url: String,
    pub implicit_wait_timeout: u64,
    pub page_load_timeout: u64,
    pub script_timeout: u64,
}

impl TestConfig {
    pub fn new() -> Self {
        Self {
            app_url: env::var("TEST_APP_URL").unwrap_or_else(|_| "http://localhost:8081".to_string()),
            webdriver_url: env::var("TEST_WEBDRIVER_URL").unwrap_or_else(|_| "http://localhost:9515".to_string()),
            implicit_wait_timeout: env::var("TEST_IMPLICIT_WAIT_TIMEOUT")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            page_load_timeout: env::var("TEST_PAGE_LOAD_TIMEOUT")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            script_timeout: env::var("TEST_SCRIPT_TIMEOUT")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
        }
    }
}

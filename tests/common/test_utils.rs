use actix_web::{
    http::{Method, Uri},
    test,
    web::Bytes,
    App, HttpRequest, HttpResponse,
};
use serde_json::json;
use std::str::FromStr;
use std::time::Duration;
use std::process::Command;
use tokio::time::sleep;
use fantoccini::{Client, ClientBuilder, Locator, elements::Element};
use url::Url;

pub const APP_URL: &str = "http://localhost:8081";

pub struct TestRequestBuilder {
    method: Method,
    path: String,
    body: Option<Bytes>,
}

impl TestRequestBuilder {
    pub fn new(method: Method, path: &str) -> Self {
        Self {
            method,
            path: path.to_string(),
            body: None,
        }
    }

    pub fn set_json(mut self, json_body: serde_json::Value) -> Self {
        self.body = Some(Bytes::from(json_body.to_string().into_bytes()));
        self
    }

    pub fn to_request(self) -> actix_web::dev::ServiceRequest {
        let uri = Uri::from_str(&format!("http://localhost{}", self.path)).unwrap();
        
        let app = App::new();
        
        let req = match self.method.as_str() {
            "GET" => test::TestRequest::get(),
            "POST" => test::TestRequest::post(),
            "PUT" => test::TestRequest::put(),
            "DELETE" => test::TestRequest::delete(),
            _ => panic!("Unsupported HTTP method"),
        }
        .uri(uri.to_string())
        .set_payload(self.body.unwrap_or_else(Bytes::new))
        .to_srv_request();
        
        req
    }
}

pub fn create_test_request(method: Method, path: &str) -> TestRequestBuilder {
    TestRequestBuilder::new(method, path)
}

pub fn create_test_request_with_body(method: Method, path: &str, body: Bytes) -> actix_web::dev::ServiceRequest {
    let uri = Uri::from_str(&format!("http://localhost{}", path)).unwrap();
    
    let req = match method.as_str() {
        "GET" => test::TestRequest::get(),
        "POST" => test::TestRequest::post(),
        "PUT" => test::TestRequest::put(),
        "DELETE" => test::TestRequest::delete(),
        _ => panic!("Unsupported HTTP method"),
    }
    .uri(uri.to_string())
    .set_payload(body)
    .to_srv_request();
    
    req
}

// Webdriver Utilities
pub async fn setup_webdriver() -> Client {
    // Kill any existing geckodriver processes
    #[cfg(target_os = "windows")]
    {
        Command::new("taskkill")
            .args(&["/F", "/IM", "geckodriver.exe"])
            .output()
            .ok();
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new("pkill")
            .arg("geckodriver")
            .output()
            .ok();
    }

    // Wait a moment for processes to terminate
    sleep(Duration::from_secs(1)).await;

    // Start WebDriver
    let client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await
        .expect("Failed to connect to WebDriver");

    client
}

pub async fn navigate_to_url(client: &Client, url: &str) {
    client.goto(url).await.expect("Failed to navigate to URL");
    sleep(Duration::from_secs(1)).await;
}

pub async fn get_current_url(client: &Client) -> String {
    client.current_url().await
        .map(|url| url.to_string())
        .expect("Failed to get current URL")
}

pub async fn find_element(client: &Client, locator: Locator<'_>) -> Option<Element> {
    match client.find(locator).await {
        Ok(element) => Some(element),
        Err(_) => None
    }
}

pub async fn click_element(client: &Client, locator: Locator<'_>) -> bool {
    match client.find(locator).await {
        Ok(element) => {
            element.click().await.is_ok()
        },
        Err(_) => false
    }
}

pub async fn input_text(client: &Client, locator: Locator<'_>, text: &str) -> bool {
    match client.find(locator).await {
        Ok(element) => {
            element.send_keys(text).await.is_ok()
        },
        Err(_) => false
    }
}

pub async fn get_error_message(client: &Client) -> Option<String> {
    find_element(client, Locator::Css(".error-message"))
        .await
        .and_then(|element| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async {
                    element.text().await.ok()
                })
        })
}

pub async fn get_success_message(client: &Client) -> Option<String> {
    find_element(client, Locator::Css(".success-message"))
        .await
        .and_then(|element| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async {
                    element.text().await.ok()
                })
        })
}

pub fn css_locator(selector: &str) -> Locator<'_> {
    Locator::Css(selector)
}

pub fn id_locator(id: &str) -> Locator<'_> {
    Locator::Id(id)
}

pub fn xpath_locator(xpath: &str) -> Locator<'_> {
    Locator::XPath(xpath)
}

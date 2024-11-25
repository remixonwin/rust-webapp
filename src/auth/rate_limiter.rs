use actix_web::{HttpRequest, HttpResponse, http::StatusCode};
use serde_json::json;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use parking_lot::Mutex;

const MAX_REQUESTS: usize = 5;
const WINDOW_DURATION: Duration = Duration::from_secs(60);

#[derive(Debug, Clone)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn get_key(req: &HttpRequest) -> String {
        // Use IP address as the rate limit key
        req.peer_addr()
            .map(|addr| addr.ip().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    }

    pub fn check_rate_limit(&self, req: &HttpRequest, email: &str) -> Result<(), HttpResponse> {
        let key = if !email.is_empty() {
            email.to_string()
        } else {
            Self::get_key(req)
        };

        let now = Instant::now();
        let mut requests = self.requests.lock();
        
        // Remove expired timestamps
        if let Some(timestamps) = requests.get_mut(&key) {
            timestamps.retain(|&t| now.duration_since(t) < WINDOW_DURATION);
            
            if timestamps.len() >= MAX_REQUESTS {
                return Err(HttpResponse::build(StatusCode::TOO_MANY_REQUESTS)
                    .json(json!({
                        "error": "Too many requests. Please try again later."
                    })));
            }
            
            timestamps.push(now);
        } else {
            requests.insert(key, vec![now]);
        }
        
        Ok(())
    }

    pub fn reset(&self, req: &HttpRequest, _email: &str) {
        let key = Self::get_key(req);
        let mut requests = self.requests.lock();
        requests.remove(&key);
    }

    pub fn cleanup(&self) {
        let now = Instant::now();
        let mut requests = self.requests.lock();
        
        requests.retain(|_, timestamps| {
            timestamps.retain(|&t| now.duration_since(t) < WINDOW_DURATION);
            !timestamps.is_empty()
        });
    }
}

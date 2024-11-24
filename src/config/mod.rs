use std::{env, net::TcpListener};

pub struct ServerConfig {
    pub host: String,
    pub port: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
        }
    }
}

impl ServerConfig {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn create_listener(&self) -> std::io::Result<TcpListener> {
        TcpListener::bind(self.address())
    }
}

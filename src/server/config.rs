use std::env;

pub struct Config {
    pub(crate) server_url: String,
}

impl Config {
    pub fn new() -> Config {
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        let server_url = format!("{}:{}", host, port);
        Config { server_url }
    }
}

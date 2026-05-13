use std::env;
use std::net::SocketAddr;

pub struct Config {
    pub(crate) kade_invoice_server_addr: SocketAddr,
}

impl Config {
    pub fn new() -> Config {
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT").unwrap_or_else(|_| "50051".to_string());
        let server_url = format!("{}:{}", host, port);
        let kade_invoice_server_addr = match server_url.parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(_) => panic!("Invalid server url: {}", server_url),
        };
        Config {
            kade_invoice_server_addr,
        }
    }
}

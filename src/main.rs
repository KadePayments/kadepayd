use crate::server::engine::Engine;
use std::process::exit;

mod data;
mod server;
mod services;

mod core;

pub mod invoice {
    tonic::include_proto!("kadepay.v1.services.invoice");
}
pub mod wallet {
    tonic::include_proto!("kadepay.v1.services.wallet");
}

#[tokio::main]
async fn main() {
    match Engine::start().await {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Server could not start: {}", error);
            exit(1)
        }
    }
}

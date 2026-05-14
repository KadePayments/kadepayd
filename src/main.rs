use crate::server::engine::Engine;

mod data;
mod server;
mod services;

pub mod invoice {
    tonic::include_proto!("kadepay.v1.services.invoice");
}

#[tokio::main]
async fn main() {
    Engine::start().await;
}

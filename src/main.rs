use crate::server::config::Config;
use crate::server::engine::Engine;

mod server;

pub mod invoice {
    tonic::include_proto!("kadepay.v1.services.invoice");
}

#[tokio::main]
async fn main() {
    let server_config = Config::new();
    Engine::start(server_config.server_url, |_| async {
        println!("Engine started!");
    })
    .await;
}

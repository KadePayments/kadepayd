use crate::server::config::Config;
use crate::server::engine::Engine;

mod data;
mod server;

#[tokio::main]
async fn main() {
    let server_config = Config::new();
    Engine::start(server_config.server_url, |_| async {
        println!("Engine started!");
    })
    .await;
}

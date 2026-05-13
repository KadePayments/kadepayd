use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;

pub struct Engine;
impl Engine {
    pub async fn start<F, Fut>(address: String, block: F)
    where
        F: Fn(TcpStream) -> Fut,
        Fut: Future + Send + 'static,
        Fut::Output: Send + 'static,
    {
        match TcpListener::bind(address.as_str()).await {
            Ok(listener) => loop {
                match listener.accept().await {
                    Ok((socket, _addr)) => {
                        spawn(block(socket));
                    }
                    Err(e) => eprintln!("error accepting socket: {}", e),
                };
            },
            Err(e) => {
                eprintln!("could not bind address: {}", e);
            }
        }
    }
}

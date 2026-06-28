use ark_core::server::Info;
use ark_grpc::{Client, Error};
use tonic::Status;

#[derive(Debug)]
pub struct ArkadeClient {
    client: Client,
}

impl ArkadeClient {
    pub async fn new_connection(url: &str) -> Result<ArkadeClient, Error> {
        let mut client = Client::new(url.to_string());
        client.connect().await?;
        Ok(Self { client })
    }

    pub async fn get_info(&self) -> Result<Info, Status> {
        match self.client.get_info().await {
            Ok(server_info) => Ok(server_info),
            Err(error) => Err(Status::from_error(Box::from(error))),
        }
    }
}

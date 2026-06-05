use crate::data::storage::Storage;
use crate::wallet::wallet_service_server::WalletService;
use crate::wallet::{NewWalletRequest, NewWalletResponse};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

#[derive(Debug)]
pub struct KadeWalletService {
    storage: Arc<Storage>,
}

impl KadeWalletService {
    pub const CREATE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS wallets (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    pub_key VARCHAR(66) NOT NULL UNIQUE
    );";

    const INSERT: &'static str = "INSERT INTO wallets (pub_key) VALUES ($1) RETURNING id;";

    const SELECT_BY_ID: &'static str = "SELECT * FROM wallets WHERE id = $1;";

    pub fn new(storage: Arc<Storage>) -> Self {
        Self { storage }
    }

    pub(crate) async fn get_wallet_pub_key(&self, id: Uuid) -> Result<String, String> {
        match self.storage.query(Self::SELECT_BY_ID, &[&id]).await {
            Ok(rows) => {
                let pub_key = match rows.first() {
                    Some(row) => row.get("pub_key"),
                    None => return Err(format!("No pubkey for id: {} was found", id)),
                };
                Ok(pub_key)
            }
            Err(err) => Err(err.to_string()),
        }
    }
}

#[tonic::async_trait]
impl WalletService for KadeWalletService {
    async fn create_wallet(
        &self,
        request: Request<NewWalletRequest>,
    ) -> Result<Response<NewWalletResponse>, Status> {
        let input = request.into_inner();
        let pub_key = input.pub_key;

        let pub_key_size = pub_key.len() / 2;
        if pub_key_size != 33 {
            return Err(Status::invalid_argument(format!(
                "PubKey must be 33 bytes: {}",
                pub_key_size
            )));
        }
        match self.storage.query_one(Self::INSERT, &[&pub_key]).await {
            Ok(row) => Ok(Response::new(NewWalletResponse::from_row(row))),
            Err(err) => Err(Status::internal(format!("{:?}", err))),
        }
    }
}

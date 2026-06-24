use crate::data::storage::Storage;
use crate::wallet::wallet_service_server::WalletService;
use crate::wallet::{NewWalletRequest, NewWalletResponse};
use bitcoin::bip32::Xpub;
use std::str::FromStr;
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
    x_pub_key VARCHAR(150) NOT NULL UNIQUE
    );";

    const INSERT: &'static str = "INSERT INTO wallets (x_pub_key) VALUES ($1) RETURNING id;";

    const SELECT_BY_ID: &'static str = "SELECT * FROM wallets WHERE id = $1;";

    pub fn new(storage: Arc<Storage>) -> Self {
        Self { storage }
    }

    pub(crate) async fn get_wallet_x_pub_key(&self, id: Uuid) -> Result<String, Status> {
        match self.storage.query(Self::SELECT_BY_ID, &[&id]).await {
            Ok(rows) => {
                let pub_key = match rows.first() {
                    Some(row) => row.get("x_pub_key"),
                    None => {
                        return Err(Status::not_found(format!(
                            "No xpubkey for id: {} was found",
                            id
                        )));
                    }
                };
                Ok(pub_key)
            }
            Err(_) => Err(Status::internal("Internal server error")),
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
        let x_pub_key = match Xpub::from_str(input.x_pub_key.as_str()) {
            Ok(x_pub) => x_pub,
            Err(_) => return Err(Status::invalid_argument("Invalid X PubKey")),
        };

        match self
            .storage
            .query_one(Self::INSERT, &[&x_pub_key.to_string()])
            .await
        {
            Ok(row) => Ok(Response::new(NewWalletResponse::from_row(row))),
            Err(error) => {
                if error.message.contains("duplicate key") || error.message.contains("23505") {
                    Err(Status::already_exists("Pubkey already exists"))
                } else {
                    eprintln!("{:?}", error);
                    Err(Status::internal("Internal server error"))
                }
            }
        }
    }
}

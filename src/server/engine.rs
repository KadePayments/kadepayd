use crate::core::arkade::ark_client::ArkadeClient;
use crate::data::errors::StorageError;
use crate::data::storage::Storage;
use crate::invoice::invoice_service_server::InvoiceServiceServer;
use crate::server::config::Config;
use crate::services::invoice_service::KadeInvoiceService;
use crate::services::wallet_service::KadeWalletService;
use crate::wallet::wallet_service_server::WalletServiceServer;
use std::sync::Arc;
use tonic::transport::Server;

pub struct Engine;
impl Engine {
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        let server_config = Config::new();
        let storage = Arc::new(Storage::new(false).await?);
        Self::init_storage(&storage).await?;
        let wallet_service = KadeWalletService::new(storage.clone());
        let ark_client =
            ArkadeClient::new_connection(server_config.arkade_server_url.as_str()).await?;
        let invoice_service = KadeInvoiceService::new(storage.clone(), ark_client);

        Server::builder()
            .add_service(InvoiceServiceServer::new(invoice_service))
            .add_service(WalletServiceServer::new(wallet_service))
            .serve(server_config.kadepay_server_addr)
            .await?;
        Ok(())
    }

    async fn init_storage(storage: &Storage) -> Result<(), StorageError> {
        let create_table_commands = [
            KadeInvoiceService::CREATE_TABLE,
            KadeWalletService::CREATE_TABLE,
            KadeInvoiceService::CREATE_CHILD_INDICES_TABLE,
        ];
        storage.init(&create_table_commands).await
    }
}

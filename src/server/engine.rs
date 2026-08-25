use crate::data::errors::StorageError;
use crate::data::storage::Storage;
use crate::invoice::invoice_service_server::InvoiceServiceServer;
use crate::server::config::Config;
use crate::server::routing::routes;
use crate::services::invoice_service::KadeInvoiceService;
use crate::services::wallet_service::KadeWalletService;
use crate::wallet::wallet_service_server::WalletServiceServer;
use axum::serve;
use std::sync::Arc;
use tonic::codec::CompressionEncoding::Gzip;
use tonic::service::Routes;

pub struct Engine;
impl Engine {
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        let server_config = Config::new();
        let storage = Arc::new(Storage::new(false).await?);
        Self::init_storage(&storage).await?;
        let wallet_service = KadeWalletService::new(storage.clone());
        let invoice_service = KadeInvoiceService::new(storage.clone());
        let wallet_server = WalletServiceServer::new(wallet_service)
            .accept_compressed(Gzip)
            .send_compressed(Gzip);
        let invoice_server = InvoiceServiceServer::new(invoice_service)
            .accept_compressed(Gzip)
            .send_compressed(Gzip);

        let grpc_router = Routes::new(invoice_server)
            .add_service(wallet_server)
            .prepare()
            .into_axum_router();

        let router = routes(server_config.clone()).await.merge(grpc_router);

        let listener = tokio::net::TcpListener::bind(server_config.kadepay_server_addr).await?;
        serve(listener, router).await?;

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

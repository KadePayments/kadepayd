use crate::data::errors::StorageError;
use crate::data::storage::Storage;
use crate::invoice::invoice_service_server::InvoiceServiceServer;
use crate::server::config::Config;
use crate::services::invoice_service::KadeInvoiceService;
use tonic::transport::Server;

pub struct Engine;
impl Engine {
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        let server_config = Config::new();
        let storage = Storage::new(false).await?;
        Self::init_storage(&storage).await?;
        let invoice_service = KadeInvoiceService::new(storage);
        Server::builder()
            .add_service(InvoiceServiceServer::new(invoice_service))
            .serve(server_config.kadepay_invoice_server_addr)
            .await?;
        Ok(())
    }

    async fn init_storage(storage: &Storage) -> Result<(), StorageError> {
        let create_table_commands = [KadeInvoiceService::CREATE_TABLE];
        storage.init(&create_table_commands).await
    }
}

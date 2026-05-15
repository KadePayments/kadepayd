use crate::data::errors::StorageError;
use crate::data::storage::Storage;
use crate::invoice::invoice_service_server::InvoiceServiceServer;
use crate::server::config::Config;
use crate::services::invoice_service::KadeInvoiceService;
use std::process::exit;
use tonic::transport::Server;

pub struct Engine;
impl Engine {
    pub async fn start() {
        let server_config = Config::new();
        match Storage::new().await {
            Ok(storage) => {
                Self::init_storage(&storage).await;

                let invoice_service = KadeInvoiceService::new(storage);
                match Server::builder()
                    .add_service(InvoiceServiceServer::new(invoice_service))
                    .serve(server_config.kade_invoice_server_addr)
                    .await
                {
                    Ok(_) => println!("Invoice server started successfully"),
                    Err(error) => eprintln!("Failed to start the invoice server: {}", error),
                }
            }
            Err(error) => {
                eprintln!("Server could not start: {}", error.message);
                exit(1)
            }
        }
    }

    async fn init_storage(storage: &Storage) -> Result<(), StorageError> {
        let create_table_commands = [KadeInvoiceService::CREATE_TABLE];
        storage.init(&create_table_commands).await
    }
}

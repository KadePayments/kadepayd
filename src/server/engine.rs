use crate::data::storage::Storage;
use crate::invoice::invoice_service_server::InvoiceServiceServer;
use crate::server::config::Config;
use crate::services::invoice_service::KadeInvoiceService;
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
            Err(error) => eprintln!("{}", error.message),
        }
    }

    async fn init_storage(storage: &Storage) {
        let create_table_commands = [KadeInvoiceService::CREATE_TABLE];
        match storage.init(&create_table_commands).await {
            Ok(_) => (),
            Err(error) => eprintln!("{}", error.message),
        }
    }
}

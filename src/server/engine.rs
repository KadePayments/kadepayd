use crate::invoice::invoice_service_server::InvoiceServiceServer;
use crate::server::config::Config;
use crate::services::invoice_service::KadeInvoiceService;
use tonic::transport::Server;

pub struct Engine;
impl Engine {
    pub async fn start() {
        let server_config = Config::new();
        let invoice_service = KadeInvoiceService::default();
        match Server::builder()
            .add_service(InvoiceServiceServer::new(invoice_service))
            .serve(server_config.kade_invoice_server_addr)
            .await
        {
            Ok(_) => println!("Invoice server started successfully"),
            Err(error) => eprintln!("Failed to start the invoice server: {}", error),
        }
    }
}

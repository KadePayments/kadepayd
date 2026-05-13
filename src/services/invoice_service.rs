use crate::invoice::invoice_service_server::InvoiceService;
use crate::invoice::{NewInvoiceRequest, NewInvoiceResponse};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct KadeInvoiceService;

#[tonic::async_trait]
impl InvoiceService for KadeInvoiceService {
    async fn create_invoice(
        &self,
        request: Request<NewInvoiceRequest>,
    ) -> Result<Response<NewInvoiceResponse>, Status> {
        println!("Got a new invoice request: {:?}", request.into_inner());

        Ok(Response::new(NewInvoiceResponse::default()))
    }
}

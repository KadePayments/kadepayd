use crate::data::storage::Storage;
use crate::invoice::invoice_service_server::InvoiceService;
use crate::invoice::{NewInvoiceRequest, NewInvoiceResponse};
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct KadeInvoiceService {
    storage: Storage,
}

impl KadeInvoiceService {
    pub const CREATE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS invoices (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    amount NUMERIC(24, 8) NOT NULL,
    currency_code VARCHAR(3) NOT NULL,
    network VARCHAR(20) NOT NULL,
    address VARCHAR(90) NOT NULL UNIQUE,
    status VARCHAR(10) NOT NULL,
    description VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
    );";
    pub const INSERT: &'static str = "INSERT INTO invoices (
    amount,
    currency_code,
    network,
    address,
    status,
    description,
    created_at
    ) VALUES ($1, $2, $3, $4, $5, $6, $7);";
    pub const SELECT_BY_ID: &'static str = "SELECT * FROM invoices WHERE id = $1;";
    pub const SELECT_BY_ADDRESS: &'static str = "SELECT * FROM invoices WHERE address = $1;";

    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

#[tonic::async_trait]
impl InvoiceService for KadeInvoiceService {
    async fn create_invoice(
        &self,
        request: Request<NewInvoiceRequest>,
    ) -> Result<Response<NewInvoiceResponse>, Status> {
        let invoice = request.into_inner();
        let address = "bc1q...".to_string();
        let status = "pending".to_string();
        let created_at = Utc::now();
        let amount = match Decimal::from_f64_retain(invoice.amount) {
            Some(amount) => amount,
            None => {
                return Err(Status::invalid_argument(
                    "cannot create an invoice without a amount",
                ));
            }
        };

        match self
            .storage
            .execute(
                Self::INSERT,
                &[
                    &amount,
                    &invoice.currency_code,
                    &invoice.network,
                    &address,
                    &status,
                    &invoice.description,
                    &created_at,
                ],
            )
            .await
        {
            Ok(_) => {
                let invoice_row = match self
                    .storage
                    .query_one(Self::SELECT_BY_ADDRESS, &[&address.as_str()])
                    .await
                {
                    Ok(value) => value,
                    Err(error) => return Err(Status::internal(error.message)),
                };
                Ok(Response::new(NewInvoiceResponse::from_row(invoice_row)))
            }
            Err(error) => Err(Status::internal(error.message)),
        }
    }
}

use crate::data::storage::Storage;
use crate::invoice::invoice_service_server::InvoiceService;
use crate::invoice::{NewInvoiceRequest, NewInvoiceResponse};
use chrono::Utc;
use rust_decimal::Decimal;
use std::str::FromStr;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

#[derive(Debug)]
pub struct KadeInvoiceService {
    storage: Arc<Storage>,
}

impl KadeInvoiceService {
    pub const CREATE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS invoices (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    pub_key_id UUID NOT NULL,
    amount NUMERIC(24, 8) NOT NULL,
    currency_code VARCHAR(3) NOT NULL,
    network VARCHAR(20) NOT NULL,
    address VARCHAR(90) NOT NULL UNIQUE,
    status VARCHAR(10) NOT NULL,
    description VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
    );";
    pub const INSERT: &'static str = "INSERT INTO invoices (
    pub_key_id,
    amount,
    currency_code,
    network,
    address,
    status,
    description,
    created_at
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;";
    pub const SELECT_BY_ID: &'static str = "SELECT * FROM invoices WHERE id = $1;";
    pub const SELECT_BY_ADDRESS: &'static str = "SELECT * FROM invoices WHERE address = $1;";

    pub fn new(storage: Arc<Storage>) -> Self {
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

        let pub_key_id = match Uuid::from_str(invoice.pub_key_id.as_str()) {
            Ok(id) => id,
            Err(error) => return Err(Status::invalid_argument(error.to_string())),
        };

        let address = if invoice.network == "Arkade" {
            "<ark1...>".to_string()
        } else {
            "<bc1q...>".to_string()
        };
        let status = "pending".to_string();
        let created_at = Utc::now();
        let amount = match Decimal::from_str(invoice.amount.as_str()) {
            Ok(amount) => amount,
            Err(error) => {
                return Err(Status::invalid_argument(format!(
                    "Invalid argument: {}",
                    error
                )));
            }
        };
        let invoice_row = match self
            .storage
            .query_one(
                Self::INSERT,
                &[
                    &pub_key_id,
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
            Ok(value) => value,
            Err(error) => {
                if error.message.contains("duplicate key") || error.message.contains("23505") {
                    return Err(Status::already_exists(
                        "Invoice with given address already exists",
                    ));
                } else {
                    eprintln!("{:?}", error);
                    return Err(Status::internal("Internal server error"));
                }
            }
        };
        Ok(Response::new(NewInvoiceResponse::from_row(invoice_row)))
    }
}

use crate::core::bitcoin::addresses::new_onchain_payment_address;
use crate::data::errors::handle_storage_error;
use crate::data::storage::Storage;
use crate::invoice::invoice_service_server::InvoiceService;
use crate::invoice::{NewInvoiceRequest, NewInvoiceResponse};
use crate::services::wallet_service::KadeWalletService;
use bitcoin::Network;
use chrono::Utc;
use rust_decimal::Decimal;
use std::str::FromStr;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

#[derive(Debug)]
pub struct KadeInvoiceService {
    storage: Arc<Storage>,
    wallet: KadeWalletService,
}

impl KadeInvoiceService {
    pub const CREATE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS invoices (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    x_pub_key_id UUID NOT NULL,
    child_key_index INT NOT NULL CHECK(child_key_index >= 0 AND child_key_index <= 2147483647),
    amount NUMERIC(24, 8) NOT NULL,
    currency_code VARCHAR(3) NOT NULL,
    chain VARCHAR(8) NOT NULL,
    network VARCHAR(20) NOT NULL,
    address VARCHAR(90) NOT NULL UNIQUE,
    status VARCHAR(10) NOT NULL,
    description VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    CONSTRAINT unique_parent_and_child UNIQUE (x_pub_key_id, child_key_index)
    );";

    pub const CREATE_CHILD_INDICES_TABLE: &'static str =
        "CREATE TABLE IF NOT EXISTS child_key_indices (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    x_pub_key_id UUID NOT NULL,
    child_key_index INT NOT NULL CHECK(child_key_index >= 0 AND child_key_index <= 2147483647));";
    pub const INSERT: &'static str = "INSERT INTO invoices (
    x_pub_key_id,
    child_key_index,
    amount,
    currency_code,
    chain,
    network,
    address,
    status,
    description,
    created_at
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *;";
    pub const INSERT_CHILD_INDEX: &'static str =
        "INSERT INTO child_key_indices (x_pub_key_id, child_key_index) VALUES ($1, $2)";
    pub const DELETE_CHILD_INDEX: &'static str =
        "DELETE FROM child_key_indices WHERE x_pub_key_id = $1 AND child_key_index = $2";
    pub const SELECT_BY_ID: &'static str = "SELECT * FROM invoices WHERE id = $1;";
    pub const SELECT_BY_ADDRESS: &'static str = "SELECT * FROM invoices WHERE address = $1;";

    pub const SELECT_MAX_CHILD_INDEX_BY_WALLET: &'static str =
        "SELECT MAX(child_key_index) FROM child_key_indices WHERE x_pub_key_id = $1;";

    pub const SELECT_CHILD_INDEX_BY_WALLET: &'static str =
        "SELECT child_key_index FROM child_key_indices WHERE x_pub_key_id = $1;";

    pub const SELECT_CHILD_INDICES_BY_WALLET: &'static str =
        "SELECT * FROM child_key_indices WHERE x_pub_key_id = $1;";

    pub fn new(storage: Arc<Storage>, wallet: KadeWalletService) -> Self {
        Self { storage, wallet }
    }

    async fn process_new_invoice_request(
        &self,
        request: Request<NewInvoiceRequest>,
    ) -> (Result<Response<NewInvoiceResponse>, (Status, Uuid, u32)>) {
        let invoice = request.into_inner();

        let x_pub_key_id = match Uuid::from_str(invoice.x_pub_key_id.as_str()) {
            Ok(id) => id,
            Err(error) => {
                return Err((Status::invalid_argument(error.to_string()), Uuid::nil(), 0));
            }
        };

        let (account_x_pub_key, new_child_key_index) = {
            let mut connection = match self.storage.pool.get().await {
                Ok(connection) => connection,
                Err(error) => {
                    eprintln!("{:?}", error);
                    return Err((Status::internal("Internal server error"), x_pub_key_id, 0));
                }
            };
            let transaction = match connection.transaction().await {
                Ok(transaction) => transaction,
                Err(err) => {
                    eprintln!("{:?}", err);
                    return Err((Status::internal("Internal server error"), x_pub_key_id, 0));
                }
            };
            let account_x_pub_key =
                match KadeWalletService::get_x_pub_key_from_db_tx(&transaction, x_pub_key_id).await
                {
                    Ok(account_x_pub_key) => account_x_pub_key,
                    Err(status) => return Err((status, x_pub_key_id, 0)),
                };

            let new_child_key_index = match Storage::tx_query_one(
                &transaction,
                Self::SELECT_MAX_CHILD_INDEX_BY_WALLET,
                &[&x_pub_key_id],
            )
            .await
            {
                Ok(prev_index_row) => {
                    let prev_index_as_option: Option<i32> = prev_index_row.get("max");
                    match prev_index_as_option {
                        Some(prev_index) => match prev_index.checked_add(1) {
                            Some(new_index) => new_index as u32,
                            None => {
                                return Err((
                                    Status::resource_exhausted("Child key indices exhausted"),
                                    x_pub_key_id,
                                    0,
                                ));
                            }
                        },
                        None => 0u32,
                    }
                }
                Err(e) => {
                    let status = handle_storage_error(e, "");
                    return Err((status, x_pub_key_id, 0));
                }
            };

            match Storage::tx_execute(
                &transaction,
                Self::INSERT_CHILD_INDEX,
                &[&x_pub_key_id, &(new_child_key_index as i32)],
            )
            .await
            {
                Ok(_) => {}
                Err(error) => {
                    let status = handle_storage_error(error, "");
                    return Err((status, x_pub_key_id, 0));
                }
            }

            match Storage::tx_commit(transaction).await {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("{:?}", error);
                    let status = handle_storage_error(error, "");
                    return Err((status, x_pub_key_id, 0));
                }
            }
            (account_x_pub_key, new_child_key_index)
        };

        let network = match Network::from_str(invoice.network.as_str()) {
            Ok(network) => network,
            Err(_) => {
                return Err((
                    Status::invalid_argument(format!(
                        "Cannot parse network with invalid name: {}",
                        invoice.network
                    )),
                    x_pub_key_id,
                    new_child_key_index,
                ));
            }
        };

        let address = if invoice.chain == "Arkade" {
            "<ark1...>".to_string()
        } else {
            match new_onchain_payment_address(
                account_x_pub_key.to_string(),
                new_child_key_index,
                network,
            ) {
                Ok(address) => address.to_string(),
                Err(status) => return Err((status, x_pub_key_id, new_child_key_index)),
            }
        };

        let status = "pending".to_string();
        let created_at = Utc::now();
        let amount = match Decimal::from_str(invoice.amount.as_str()) {
            Ok(amount) => amount,
            Err(error) => {
                eprintln!("{:?}", error);
                return Err((
                    Status::invalid_argument("Invalid argument"),
                    x_pub_key_id,
                    new_child_key_index,
                ));
            }
        };

        let invoice_row = match self
            .storage
            .query_one(
                Self::INSERT,
                &[
                    &x_pub_key_id,
                    &(new_child_key_index as i32),
                    &amount,
                    &invoice.currency_code,
                    &invoice.chain,
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
                let status =
                    handle_storage_error(error, "Invoice with given address already exists");
                return Err((status, x_pub_key_id, new_child_key_index));
            }
        };

        Ok(Response::new(NewInvoiceResponse::from_row(invoice_row)))
    }
}

#[tonic::async_trait]
impl InvoiceService for KadeInvoiceService {
    async fn create_invoice(
        &self,
        request: Request<NewInvoiceRequest>,
    ) -> Result<Response<NewInvoiceResponse>, Status> {
        match self.process_new_invoice_request(request).await {
            Ok(response) => Ok(response),
            Err((status, x_pub_key_id, new_child_key_index)) => {
                match self
                    .storage
                    .execute(
                        Self::DELETE_CHILD_INDEX,
                        &[&x_pub_key_id, &(new_child_key_index as i32)],
                    )
                    .await
                {
                    Ok(_) => {}
                    Err(error) => {
                        let status = handle_storage_error(error, "");
                        return Err(status);
                    }
                }
                Err(status)
            }
        }
    }
}

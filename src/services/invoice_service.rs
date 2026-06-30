use crate::core::KadeHDWallet;
use crate::core::arkade::ark_client::ArkadeClient;
use crate::data::errors::handle_storage_error;
use crate::data::storage::Storage;
use crate::invoice::invoice_service_server::InvoiceService;
use crate::invoice::{GetInvoicesRequest, GetInvoicesResponse, InvoiceResponse, NewInvoiceRequest};
use crate::server::config::Config;
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
    test: bool,
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
    address VARCHAR(150) NOT NULL UNIQUE,
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
    pub const SELECT_BY_WALLET: &'static str = "SELECT * FROM invoices WHERE x_pub_key_id = $1;";
    pub const SELECT_BY_ADDRESS: &'static str = "SELECT * FROM invoices WHERE address = $1;";

    pub const SELECT_MAX_CHILD_INDEX_BY_WALLET: &'static str =
        "SELECT MAX(child_key_index) FROM child_key_indices WHERE x_pub_key_id = $1;";

    pub const SELECT_CHILD_INDEX_BY_WALLET: &'static str =
        "SELECT child_key_index FROM child_key_indices WHERE x_pub_key_id = $1;";

    pub const SELECT_CHILD_INDICES_BY_WALLET: &'static str =
        "SELECT * FROM child_key_indices WHERE x_pub_key_id = $1;";

    pub fn new(storage: Arc<Storage>) -> Self {
        Self {
            storage,
            test: false,
        }
    }

    pub fn new_test(storage: Arc<Storage>) -> Self {
        Self {
            storage,
            test: true,
        }
    }

    async fn process_new_invoice_request(
        &self,
        request: Request<NewInvoiceRequest>,
    ) -> (Result<Response<InvoiceResponse>, (Status, Option<(Uuid, u32)>)>) {
        let invoice = request.into_inner();

        let x_pub_key_id = match Uuid::from_str(invoice.x_pub_key_id.as_str()) {
            Ok(id) => id,
            Err(error) => {
                return Err((Status::invalid_argument(error.to_string()), None));
            }
        };

        let (account_x_pub_key, new_child_key_index) = {
            let mut connection = match self.storage.pool.get().await {
                Ok(connection) => connection,
                Err(error) => {
                    eprintln!("{:?}", error);
                    return Err((Status::internal("Internal server error"), None));
                }
            };
            let transaction = match connection.transaction().await {
                Ok(transaction) => transaction,
                Err(err) => {
                    eprintln!("{:?}", err);
                    return Err((Status::internal("Internal server error"), None));
                }
            };
            let account_x_pub_key =
                match KadeWalletService::get_x_pub_key_from_db_tx(&transaction, x_pub_key_id).await
                {
                    Ok(account_x_pub_key) => account_x_pub_key,
                    Err(status) => return Err((status, None)),
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
                                    None,
                                ));
                            }
                        },
                        None => 0u32,
                    }
                }
                Err(e) => {
                    let status = handle_storage_error(e, "");
                    return Err((status, None));
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
                    return Err((status, None));
                }
            }

            match Storage::tx_commit(transaction).await {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("{:?}", error);
                    let status = handle_storage_error(error, "");
                    return Err((status, None));
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
                    Some((x_pub_key_id, new_child_key_index)),
                ));
            }
        };

        let address = if invoice.chain == "Arkade" {
            let server_info = {
                if self.test {
                    ArkadeClient::get_test_info()
                } else {
                    let server_config = Config::new();
                    let arkade_client = match ArkadeClient::new_connection(
                        server_config.arkade_server_url.as_str(),
                    )
                    .await
                    {
                        Ok(client) => client,
                        Err(error) => {
                            return Err((
                                Status::from_error(Box::from(error)),
                                Some((x_pub_key_id, new_child_key_index)),
                            ));
                        }
                    };
                    match arkade_client.get_info().await {
                        Ok(server_info) => server_info,
                        Err(status) => {
                            return Err((status, Some((x_pub_key_id, new_child_key_index))));
                        }
                    }
                }
            };

            let ark_network = server_info.network;
            if ark_network != network {
                return Err((
                    Status::invalid_argument(format!(
                        "Arkade server network {ark_network} does not match invoice network {network}"
                    )),
                    Some((x_pub_key_id, new_child_key_index)),
                ));
            }

            let server_pub_key = server_info.signer_pk.x_only_public_key().0;
            let exit_delay = server_info.unilateral_exit_delay;
            match KadeHDWallet::new_offchain_payment_address(
                account_x_pub_key,
                server_pub_key,
                exit_delay,
                new_child_key_index,
                network,
            ) {
                Ok(address) => address.to_string(),
                Err(status) => return Err((status, Some((x_pub_key_id, new_child_key_index)))),
            }
        } else {
            match KadeHDWallet::new_onchain_payment_address(
                account_x_pub_key.to_string(),
                new_child_key_index,
                network,
            ) {
                Ok(address) => address.to_string(),
                Err(status) => return Err((status, Some((x_pub_key_id, new_child_key_index)))),
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
                    Some((x_pub_key_id, new_child_key_index)),
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
                return Err((status, Some((x_pub_key_id, new_child_key_index))));
            }
        };

        Ok(Response::new(InvoiceResponse::from_row(&invoice_row)))
    }
}

#[tonic::async_trait]
impl InvoiceService for KadeInvoiceService {
    async fn create_invoice(
        &self,
        request: Request<NewInvoiceRequest>,
    ) -> Result<Response<InvoiceResponse>, Status> {
        match self.process_new_invoice_request(request).await {
            Ok(response) => Ok(response),
            Err((status, Some((x_pub_key_id, new_child_key_index)))) => {
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
            Err((status, None)) => Err(status),
        }
    }

    async fn get_invoices(
        &self,
        request: Request<GetInvoicesRequest>,
    ) -> Result<Response<GetInvoicesResponse>, Status> {
        let x_pub_key_id = match Uuid::from_str(request.into_inner().x_pub_key_id.as_str()) {
            Ok(x_pub_key_id) => x_pub_key_id,
            Err(_) => return Err(Status::invalid_argument("Invalid x-pub-key id")),
        };

        let invoices: Vec<InvoiceResponse> = match self
            .storage
            .query(Self::SELECT_BY_WALLET, &[&x_pub_key_id])
            .await
        {
            Ok(rows) => rows
                .iter()
                .map(|row| InvoiceResponse::from_row(row))
                .collect(),
            Err(error) => {
                let status = handle_storage_error(error, "");
                return Err(status);
            }
        };

        let invoices_response = GetInvoicesResponse { invoices };
        Ok(Response::new(invoices_response))
    }
}

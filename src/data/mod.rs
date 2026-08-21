use crate::invoice::{InvoiceResponse, NewInvoiceRequest};
use crate::wallet::{NewWalletResponse, WalletIdResponse};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::from_value;
use std::collections::HashMap;
use tokio_postgres::Row;
use tonic::Status;
use uuid::Uuid;

pub mod errors;
pub mod storage;

impl InvoiceResponse {
    pub fn from_row(row: &Row) -> Result<Self, Status> {
        let id: Uuid = row.get("id");
        let x_pub_key_id: Uuid = row.get("x_pub_key_id");
        let created_at: DateTime<Utc> = row.get("created_at");
        let amount: Decimal = row.get("amount");
        let child_key_index: i32 = row.get("child_key_index");
        let metadata_value: Option<serde_json::Value> = row.get("metadata");
        let metadata: HashMap<String, String> = match metadata_value {
            Some(metadata) => match from_value(metadata) {
                Ok(metadata) => metadata,
                Err(_) => return Err(Status::internal("Failed to read invoice metadata")),
            },
            None => HashMap::new(),
        };
        Ok(Self {
            id: id.to_string(),
            x_pub_key_id: x_pub_key_id.to_string(),
            chain: row.get("chain"),
            amount: amount.to_string(),
            currency_code: row.get("currency_code"),
            network: row.get("network"),
            address: row.get("address"),
            status: row.get("status"),
            description: row.get("description"),
            metadata,
            created_at: created_at.timestamp(),
            child_key_index,
        })
    }
}

impl NewWalletResponse {
    pub fn from_row(row: Row) -> Self {
        let x_pub_key_id: Uuid = row.get("id");
        Self {
            x_pub_key_id: x_pub_key_id.to_string(),
        }
    }
}

impl WalletIdResponse {
    pub fn from_row(row: Row) -> Self {
        let wallet_id: Uuid = row.get("id");
        Self {
            wallet_id: wallet_id.to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct NewPaymentRequest {
    pub x_pub_key_id: String,
    pub chain: String,
    pub network: String,
    pub currency_code: String,
    pub amount: String,
    pub description: String,
    pub metadata: HashMap<String, String>,
}

impl NewPaymentRequest {
    pub fn to_invoice_request(&self) -> NewInvoiceRequest {
        NewInvoiceRequest {
            x_pub_key_id: self.x_pub_key_id.to_string(),
            chain: self.chain.to_string(),
            network: self.network.to_string(),
            currency_code: self.currency_code.to_string(),
            amount: self.amount.to_string(),
            description: self.description.to_string(),
            metadata: self.metadata.clone(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct NewInvoiceResponse {
    pub id: String,
    pub x_pub_key_id: String,
    pub chain: String,
    pub network: String,
    pub currency_code: String,
    pub amount: String,
    pub address: String,
    pub created_at: i64,
    pub description: String,
    pub status: String,
    pub metadata: HashMap<String, String>,
}

impl NewInvoiceResponse {
    pub fn from_response(invoice: InvoiceResponse) -> Self {
        NewInvoiceResponse {
            id: invoice.id.to_string(),
            x_pub_key_id: invoice.x_pub_key_id.to_string(),
            chain: invoice.chain.to_string(),
            network: invoice.network.to_string(),
            currency_code: invoice.currency_code.to_string(),
            amount: invoice.amount.to_string(),
            address: invoice.address.to_string(),
            created_at: invoice.created_at,
            description: invoice.description.to_string(),
            status: invoice.status.to_string(),
            metadata: invoice.metadata,
        }
    }
}

#[derive(Deserialize)]
pub struct NewInvoiceQuery {
    pub id: String,
}

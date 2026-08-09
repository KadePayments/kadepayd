use crate::invoice::InvoiceResponse;
use crate::wallet::{NewWalletResponse, WalletIdResponse};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
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

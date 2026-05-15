use crate::invoice::NewInvoiceResponse;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use tokio_postgres::Row;
use uuid::Uuid;

mod config;
mod errors;
pub mod storage;

impl NewInvoiceResponse {
    pub fn from_row(row: Row) -> Self {
        let id: Uuid = row.get("id");
        let created_at: DateTime<Utc> = row.get("created_at");
        let amount: Decimal = row.get("amount");
        Self {
            id: id.to_string(),
            amount: amount.as_f64(),
            currency_code: row.get("currency_code"),
            network: row.get("network"),
            address: row.get("address"),
            status: row.get("status"),
            description: row.get("description"),
            created_at: created_at.timestamp(),
        }
    }
}

pub mod core;
pub mod data;
mod server;
pub mod services;

pub mod invoice {
    tonic::include_proto!("kadepay.v1.services.invoice");
}
pub mod wallet {
    tonic::include_proto!("kadepay.v1.services.wallet");
}

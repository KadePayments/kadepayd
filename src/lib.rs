pub mod data;
pub mod services;

pub mod invoice {
    tonic::include_proto!("kadepay.v1.services.invoice");
}

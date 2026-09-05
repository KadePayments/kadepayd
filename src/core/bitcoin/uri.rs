use bip321::Bip321Uri;
use bitcoin::{Address, Amount};
use rust_decimal::Decimal;
use std::str::FromStr;
use tonic::Status;

pub fn encode_bitcoin_uri(
    address: &String,
    amount: &Decimal,
    label: &String,
    message: &String,
    network: &String,
) -> Result<String, Status> {
    let mut uri: Bip321Uri = Bip321Uri::new();

    let address = match Address::from_str(address) {
        Ok(address) => address,
        Err(_) => return Err(Status::invalid_argument("invalid address")),
    };

    match Amount::from_btc(amount.as_f64()) {
        Ok(amount) => match uri.set_amount(amount) {
            Ok(_) => {}
            Err(_) => return Err(Status::invalid_argument("invalid amount")),
        },
        Err(_) => return Err(Status::invalid_argument("invalid amount")),
    }

    uri.set_label(label.to_string());
    uri.set_message(message.to_string());

    let is_testnet: bool = network == "testnet"
        || network == "signet"
        || network == "testnet4"
        || network == "regtest";
    if is_testnet {
        match uri.push_tb(address, false) {
            Ok(_) => {}
            Err(_) => return Err(Status::invalid_argument("invalid testnet address")),
        }
    } else {
        match uri.set_address(address) {
            Ok(_) => {}
            Err(_) => return Err(Status::invalid_argument("invalid address")),
        };
    }

    match uri.checked_uppercase() {
        Some(uppercase_uri) => Ok(uppercase_uri),
        None => Ok(uri.to_string()),
    }
}

use bitcoin::bip32::{ChildNumber, Error, Xpub};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::{Address, Network};
use std::str::FromStr;
use tonic::Status;

pub fn new_onchain_payment_address(
    xpubkey: String,
    prev_index: u32,
    network: Network,
) -> Result<Address, Status> {
    let secp = Secp256k1::new();
    let path = match (
        ChildNumber::from_normal_idx(0),
        ChildNumber::from_normal_idx(prev_index),
    ) {
        (Ok(index), Ok(index1)) => [index, index1],
        (Ok(index), Err(_)) => {
            return Err(Status::invalid_argument(format!(
                "Invalid child number: {}",
                index
            )));
        }
        (Err(_), Ok(index)) => {
            return Err(Status::invalid_argument(format!(
                "Invalid child number: {}",
                index
            )));
        }
        (Err(_), Err(_)) => return Err(Status::invalid_argument("Invalid child numbers")),
    };
    let parent_xpub = match Xpub::from_str(xpubkey.as_str()) {
        Ok(x_pub) => x_pub,
        Err(_) => return Err(Status::invalid_argument("Invalid xpubkey")),
    };
    let child_xpub = match parent_xpub.derive_pub(&secp, &path) {
        Ok(xpub) => xpub,
        Err(_) => return Err(Status::internal("Failed to derive child xpubkey")),
    };
    let internal_key = child_xpub.to_x_only_pub();
    let address = Address::p2tr(&secp, internal_key, None, network);
    Ok(address)
}

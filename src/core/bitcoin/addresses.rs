use bitcoin::bip32::{ChildNumber, Xpub};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::{Address, Network};
use std::str::FromStr;

pub fn new_onchain_payment_address(xpubkey: String, prev_index: u32, network: Network) -> Address {
    let secp = Secp256k1::new();
    let path = [
        ChildNumber::from_normal_idx(0).unwrap(),
        ChildNumber::from_normal_idx(prev_index).unwrap(),
    ];
    let parent_xpub = Xpub::from_str(xpubkey.as_str()).unwrap();
    let child_xpub = parent_xpub.derive_pub(&secp, &path).unwrap();
    let internal_key = child_xpub.to_x_only_pub();
    let address = Address::p2tr(&secp, internal_key, None, network);
    address
}

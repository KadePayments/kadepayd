use crate::core::KadeHDWallet;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::{Address, Network};
use tonic::Status;

impl KadeHDWallet {
    pub fn new_onchain_payment_address(
        x_pub_key: String,
        prev_index: u32,
        network: Network,
    ) -> Result<Address, Status> {
        let secp = Secp256k1::new();
        let internal_key = KadeHDWallet::derive_child_key(&secp, x_pub_key, prev_index)?;
        let address = Address::p2tr(&secp, internal_key, None, network);
        Ok(address)
    }
}

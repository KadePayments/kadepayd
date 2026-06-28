use crate::core::KadeHDWallet;
use ark_core::{ArkAddress, BoardingOutput};
use bitcoin::key::Secp256k1;
use bitcoin::{Network, Sequence, XOnlyPublicKey};
use tonic::Status;

impl KadeHDWallet {
    pub fn new_offchain_payment_address(
        owner_x_pub_key: String,
        server_pubkey: XOnlyPublicKey,
        exit_delay: Sequence,
        prev_index: u32,
        network: Network,
    ) -> Result<ArkAddress, Status> {
        let secp = Secp256k1::new();
        let owner_pubkey = KadeHDWallet::derive_child_key(&secp, owner_x_pub_key, prev_index)?;

        let boarding_output =
            match BoardingOutput::new(&secp, server_pubkey, owner_pubkey, exit_delay, network) {
                Ok(boarding_output) => boarding_output,
                Err(_) => return Err(Status::internal("Internal server error")),
            };

        Ok(boarding_output.to_ark_address(network, server_pubkey))
    }
}

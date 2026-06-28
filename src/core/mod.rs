use ::bitcoin::XOnlyPublicKey;
use ::bitcoin::bip32::{ChildNumber, Xpub};
use ::bitcoin::key::Secp256k1;
use ::bitcoin::secp256k1::All;
use std::str::FromStr;
use tonic::Status;

pub mod arkade;
pub mod bitcoin;

pub struct KadeHDWallet;
impl KadeHDWallet {
    fn derive_child_key(
        secp: &Secp256k1<All>,
        x_pub_key: String,
        prev_index: u32,
    ) -> Result<XOnlyPublicKey, Status> {
        let account_index = ChildNumber::from_normal_idx(0)
            .map_err(|_| Status::internal("Failed to create account index: 0"))?;
        let child_index = ChildNumber::from_normal_idx(prev_index)
            .map_err(|_| Status::internal(format!("Invalid child number: {}", prev_index)))?;
        let path = [account_index, child_index];

        let parent_xpub = match Xpub::from_str(x_pub_key.as_str()) {
            Ok(x_pub) => x_pub,
            Err(_) => return Err(Status::invalid_argument("Invalid xpubkey")),
        };
        let child_xpub = match parent_xpub.derive_pub(&secp, &path) {
            Ok(xpub) => xpub,
            Err(_) => return Err(Status::internal("Failed to derive child xpubkey")),
        };
        Ok(child_xpub.to_x_only_pub())
    }
}

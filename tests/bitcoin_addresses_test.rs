use bitcoin::Network;
use kadepayd::core::KadeHDWallet;
use std::collections::HashSet;

#[test]
fn should_generate_new_onchain_payment_address_for_every_index_successfully() {
    let mut prev_address = "".to_string();
    let mut seen_addresses: HashSet<String> = HashSet::new();
    for index in 0..16 {
        let address_result = KadeHDWallet::new_onchain_payment_address(
            "tpubDDneEXG899zhkpQt6bqo7fmaSVVi7ErfjNSs82gmTKJHJM5dfzT6f4er8dqgt85z3TYZYzJ7FZeTzKSkX1KKs8ejtXGg4FudTA9TR55ntaF".to_string(),
            index,
            Network::Testnet,
        );
        assert!(address_result.is_ok());
        let address = address_result.unwrap().to_string();
        assert!(!address.is_empty());
        assert_ne!(address, prev_address);
        assert!(seen_addresses.insert(address.clone()));
        prev_address = address;
    }
}

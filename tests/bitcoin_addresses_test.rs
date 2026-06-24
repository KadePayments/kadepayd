use bitcoin::Network;
use kadepayd::core::bitcoin::addresses::new_onchain_payment_address;

#[test]
fn should_generate_new_onchain_payment_address_for_every_index_successfully() {
    for prev_index in 0..16 {
        let address = new_onchain_payment_address(
            "tpubDDneEXG899zhkpQt6bqo7fmaSVVi7ErfjNSs82gmTKJHJM5dfzT6f4er8dqgt85z3TYZYzJ7FZeTzKSkX1KKs8ejtXGg4FudTA9TR55ntaF".to_string(),
            prev_index,
            Network::Testnet,
        );
        eprintln!("{}: {}", prev_index, address);
    }
}

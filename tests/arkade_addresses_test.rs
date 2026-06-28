use kadepayd::core::KadeHDWallet;
use kadepayd::core::arkade::ark_client::ArkadeClient;
use std::collections::HashSet;

#[tokio::test]
async fn should_generate_new_offchain_payment_address_on_mutinynet_successfully() {
    let server_info = ArkadeClient::get_test_info();
    let server_pub_key = server_info.signer_pk.x_only_public_key().0;
    let exit_delay = server_info.unilateral_exit_delay;
    let network = server_info.network;
    let address = KadeHDWallet::new_offchain_payment_address(
        "tpubDDneEXG899zhkpQt6bqo7fmaSVVi7ErfjNSs82gmTKJHJM5dfzT6f4er8dqgt85z3TYZYzJ7FZeTzKSkX1KKs8ejtXGg4FudTA9TR55ntaF".to_string(),
        server_pub_key,
        exit_delay,
        0,
        network
    ).expect("failed to create address");

    assert_eq!(
        address.to_string(),
        "tark1qzsexy9fnys8m0v6q0fq7ey7xlr6279q0467d7se4gln8lrtz43zcksgp8pqgml42pxjjhgv50q3ruepn355r5kypk8j7jg4d04fle9kpuect8"
    );
}

#[tokio::test]
async fn should_generate_new_offchain_payment_address_for_every_index_successfully() {
    let server_info = ArkadeClient::get_test_info();
    let server_pub_key = server_info.signer_pk.x_only_public_key().0;
    let exit_delay = server_info.unilateral_exit_delay;
    let network = server_info.network;

    let mut prev_address = "".to_string();
    let mut seen_addresses: HashSet<String> = HashSet::new();
    for index in 0..16 {
        let address_result = KadeHDWallet::new_offchain_payment_address(
            "tpubDDneEXG899zhkpQt6bqo7fmaSVVi7ErfjNSs82gmTKJHJM5dfzT6f4er8dqgt85z3TYZYzJ7FZeTzKSkX1KKs8ejtXGg4FudTA9TR55ntaF".to_string(),
            server_pub_key,
            exit_delay,
            index,
            network
        );
        assert!(address_result.is_ok());
        let address = address_result.unwrap().to_string();
        assert!(!address.is_empty());
        assert_ne!(address, prev_address);
        assert!(seen_addresses.insert(address.clone()));

        prev_address = address;
    }
}

use kadepayd::core::KadeHDWallet;
use kadepayd::core::arkade::ark_client::ArkadeClient;
use std::collections::HashSet;

#[tokio::test]
async fn should_generate_new_offchain_payment_address_on_mainnet_successfully() {
    let client = ArkadeClient::new_connection("https://arkade.computer")
        .await
        .expect("failed to create client");
    let server_info = client.get_info().await.expect("failed to get server info");
    let server_pub_key = server_info.signer_pk.x_only_public_key().0;
    let exit_delay = server_info.unilateral_exit_delay;
    let network = server_info.network;
    let address = KadeHDWallet::new_offchain_payment_address(
        "xpub6CSpAb82WybGx6gtqNyncMaDrNduwq57atf5csvz2RyuCi6YU3BRupQwS25mWeM2ueuuTLp7N9UVFoefBkEBGhwux3AcBqbqfZiqa24Jc8B".to_string(),
        server_pub_key,
        exit_delay,
        0,
        network
    ).expect("failed to create address");

    assert_eq!(
        address.to_string(),
        "ark1qzpq904am6clw3pgqwyh4p02708fy4xs0hcpwt7rwfdttuxsjamecgf8g8ks09lpyl0csjk600jz0de9wmrnnz6dspdp28j9ljwcmgumkl5f6u"
    );
}

#[tokio::test]
async fn should_generate_new_offchain_payment_address_on_mutinynet_successfully() {
    let client = ArkadeClient::new_connection("https://mutinynet.arkade.sh")
        .await
        .expect("failed to create client");
    let server_info = client.get_info().await.expect("failed to get server info");
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
        "tark1qqcpq7yq3e8hhsx6ml3fud93m7827qggaurtzu3zwsr4a0qs0gf85ahfs98t7nxntywfnsxle4ugpmjcgs8pa7ntm2d80ve7qqcjfyxph33xns"
    );
}

#[tokio::test]
async fn should_generate_new_offchain_payment_address_for_every_index_successfully() {
    let client = ArkadeClient::new_connection("https://mutinynet.arkade.sh")
        .await
        .expect("failed to create client");
    let server_info = client.get_info().await.expect("failed to get server info");
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

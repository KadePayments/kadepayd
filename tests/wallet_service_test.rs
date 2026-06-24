use kadepayd::data::storage::Storage;
use kadepayd::services::wallet_service::KadeWalletService;
use kadepayd::wallet::NewWalletRequest;
use kadepayd::wallet::wallet_service_server::WalletService;
use std::sync::Arc;
use tonic::{Code, Request};

#[tokio::test]
async fn should_create_watch_only_wallet_successfully() {
    let x_pub_key = "tpubDDneEXG899zhkpQt6bqo7fmaSVVi7ErfjNSs82gmTKJHJM5dfzT6f4er8dqgt85z3TYZYzJ7FZeTzKSkX1KKs8ejtXGg4FudTA9TR55ntaF".to_string();

    let storage = Arc::new(Storage::new(true).await.expect("storage creation failed"));

    storage
        .init(&[KadeWalletService::CREATE_TABLE])
        .await
        .expect("storage initialization failed");

    let wallet_service = KadeWalletService::new(storage);

    let new_wallet_request = NewWalletRequest { x_pub_key };

    let grpc_request = Request::new(new_wallet_request);

    let new_wallet_response = wallet_service
        .create_wallet(grpc_request)
        .await
        .expect("failed to create new wallet")
        .into_inner();

    assert_eq!(!new_wallet_response.x_pub_key_id.is_empty(), true)
}

#[tokio::test]
async fn should_fail_to_create_watch_only_wallet_on_invalid_pub_key() {
    let x_pub_key = "tpubDDneEXG899zhkpQt6bqo7fmaSVVi7ErfjNSs82gmTKJHJM5dfzT6f4er8dqgt85z3TYZYzJ7FZeTzKSkX1KKs8ejtXGg4FudTA9TR55nt".to_string();

    let storage = Arc::new(Storage::new(true).await.expect("storage creation failed"));

    storage
        .init(&[KadeWalletService::CREATE_TABLE])
        .await
        .expect("storage initialization failed");

    let wallet_service = KadeWalletService::new(storage);

    let new_wallet_request = NewWalletRequest { x_pub_key };

    let grpc_request = Request::new(new_wallet_request);

    let new_wallet_response = wallet_service
        .create_wallet(grpc_request)
        .await
        .err()
        .expect("created new wallet");

    assert_eq!(new_wallet_response.code(), Code::InvalidArgument);
}

use kadepayd::data::storage::Storage;
use kadepayd::services::wallet_service::KadeWalletService;
use kadepayd::wallet::NewWalletRequest;
use kadepayd::wallet::wallet_service_server::WalletService;
use std::sync::Arc;
use tonic::{Code, Request};

#[tokio::test]
async fn should_create_watch_only_wallet_successfully() {
    let pub_key = "038d03c224f037eabaaba23719821c909e6fb25c29510b130f2888ad5f863635b5".to_string();

    let storage = Arc::new(Storage::new(true).await.expect("storage creation failed"));

    storage
        .init(&[KadeWalletService::CREATE_TABLE])
        .await
        .expect("storage initialization failed");

    let wallet_service = KadeWalletService::new(storage);

    let new_wallet_request = NewWalletRequest { pub_key };

    let grpc_request = Request::new(new_wallet_request);

    let new_wallet_response = wallet_service
        .create_wallet(grpc_request)
        .await
        .expect("failed to create new wallet")
        .into_inner();

    assert_eq!(!new_wallet_response.pub_key_id.is_empty(), true)
}

#[tokio::test]
async fn should_fail_to_create_watch_only_wallet_on_invalid_pub_key() {
    let pub_key = "03c224f037eabaaba23719821c909e6fb25c29510b130f2888ad5f863635b5".to_string();
    let pub_key_size = pub_key.len();

    let storage = Arc::new(Storage::new(true).await.expect("storage creation failed"));

    storage
        .init(&[KadeWalletService::CREATE_TABLE])
        .await
        .expect("storage initialization failed");

    let wallet_service = KadeWalletService::new(storage);

    let new_wallet_request = NewWalletRequest { pub_key };

    let grpc_request = Request::new(new_wallet_request);

    let new_wallet_response = wallet_service
        .create_wallet(grpc_request)
        .await
        .err()
        .expect("created new wallet");

    assert_eq!(new_wallet_response.code(), Code::InvalidArgument);
    assert_eq!(
        new_wallet_response.message(),
        format!("PubKey must be 33 bytes: {}", pub_key_size / 2)
    );
}

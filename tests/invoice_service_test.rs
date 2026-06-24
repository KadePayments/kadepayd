use kadepayd::data::storage::Storage;
use kadepayd::invoice::NewInvoiceRequest;
use kadepayd::invoice::invoice_service_server::InvoiceService;
use kadepayd::services::invoice_service::KadeInvoiceService;
use kadepayd::services::wallet_service::KadeWalletService;
use kadepayd::wallet::NewWalletRequest;
use kadepayd::wallet::wallet_service_server::WalletService;
use std::sync::Arc;
use tonic::Request;

#[tokio::test]
async fn should_create_an_invoice_successfully() {
    let storage = Arc::new(Storage::new(true).await.expect("storage creation failed"));

    storage
        .init(&[
            KadeInvoiceService::CREATE_TABLE,
            KadeWalletService::CREATE_TABLE,
        ])
        .await
        .expect("storage initialization failed");

    let wallet = KadeWalletService::new(storage.clone());
    let  wallet_req = NewWalletRequest { x_pub_key: "tpubDDneEXG899zhkpQt6bqo7fmaSVVi7ErfjNSs82gmTKJHJM5dfzT6f4er8dqgt85z3TYZYzJ7FZeTzKSkX1KKs8ejtXGg4FudTA9TR55ntaF".to_string() };
    let grpc_req = Request::new(wallet_req);
    let new_wallet_res = wallet
        .create_wallet(grpc_req)
        .await
        .expect("failed to create wallet")
        .into_inner();

    let invoice_service = KadeInvoiceService::new(storage, wallet);

    let invoice_req = NewInvoiceRequest {
        x_pub_key_id: new_wallet_res.x_pub_key_id.to_string(),
        chain: "Bitcoin".to_string(),
        network: "testnet".to_string(),
        currency_code: "BTC".to_string(),
        amount: "0.0034".to_string(),
        description: "Create an invoice on Bitcoin test".to_string(),
    };

    let grpc_req = Request::new(invoice_req);

    let new_invoice_res = invoice_service
        .create_invoice(grpc_req)
        .await
        .expect("failed to create new invoice")
        .into_inner();

    assert_eq!(
        new_invoice_res.x_pub_key_id,
        new_wallet_res.x_pub_key_id.to_string()
    );
    assert_eq!(new_invoice_res.amount, "0.00340000");
    assert_eq!(
        new_invoice_res.description,
        "Create an invoice on Bitcoin test"
    );
    assert_eq!(new_invoice_res.chain, "Bitcoin");
    assert_eq!(new_invoice_res.currency_code, "BTC");
    assert!(
        !new_invoice_res.address.is_empty(),
        "expect a non-empty invoice address"
    );
    assert_eq!(new_invoice_res.status, "pending");
    assert!(new_invoice_res.created_at > 0)
}

#[tokio::test]
async fn should_create_new_onchain_payment_address_for_every_new_invoice_successfully() {
    let storage = Arc::new(Storage::new(true).await.expect("storage creation failed"));

    storage
        .init(&[
            KadeInvoiceService::CREATE_TABLE,
            KadeWalletService::CREATE_TABLE,
        ])
        .await
        .expect("storage initialization failed");

    let wallet = KadeWalletService::new(storage.clone());
    let  wallet_req = NewWalletRequest { x_pub_key: "tpubDDneEXG899zhkpQt6bqo7fmaSVVi7ErfjNSs82gmTKJHJM5dfzT6f4er8dqgt85z3TYZYzJ7FZeTzKSkX1KKs8ejtXGg4FudTA9TR55ntaF".to_string() };
    let grpc_req = Request::new(wallet_req);
    let new_wallet_res = wallet
        .create_wallet(grpc_req)
        .await
        .expect("failed to create wallet")
        .into_inner();

    let invoice_service = KadeInvoiceService::new(storage, wallet);

    let mut prev_address = "".to_string();

    for index in 0..16 {
        let invoice_req = NewInvoiceRequest {
            x_pub_key_id: new_wallet_res.x_pub_key_id.to_string(),
            chain: "Bitcoin".to_string(),
            network: "testnet".to_string(),
            currency_code: "BTC".to_string(),
            amount: "0.0034".to_string(),
            description: "Create an invoice on Bitcoin test".to_string(),
        };

        let grpc_req = Request::new(invoice_req);

        let new_invoice_res = invoice_service
            .create_invoice(grpc_req)
            .await
            .expect("failed to create new invoice")
            .into_inner();

        assert_eq!(
            new_invoice_res.x_pub_key_id,
            new_wallet_res.x_pub_key_id.to_string()
        );
        assert_eq!(new_invoice_res.amount, "0.00340000");
        assert_eq!(
            new_invoice_res.description,
            "Create an invoice on Bitcoin test"
        );
        assert_eq!(new_invoice_res.chain, "Bitcoin");
        assert_eq!(new_invoice_res.currency_code, "BTC");
        assert!(
            !new_invoice_res.address.is_empty(),
            "expect a non-empty invoice address"
        );
        assert_ne!(
            new_invoice_res.address, prev_address,
            "expect the new invoice payment address to be different from the previous invoice payment address"
        );
        assert_eq!(new_invoice_res.status, "pending");
        assert!(new_invoice_res.created_at > 0);
        prev_address = new_invoice_res.address;
    }
}

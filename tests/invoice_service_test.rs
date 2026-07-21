use ark_core::ArkAddress;
use bitcoin::{Address, Network};
use kadepayd::data::storage::Storage;
use kadepayd::invoice::invoice_service_server::InvoiceService;
use kadepayd::invoice::{GetInvoicesRequest, NewInvoiceRequest};
use kadepayd::services::invoice_service::KadeInvoiceService;
use kadepayd::services::wallet_service::KadeWalletService;
use kadepayd::wallet::NewWalletRequest;
use kadepayd::wallet::wallet_service_server::WalletService;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use tonic::Request;
use uuid::Uuid;

fn get_invoice_metadata() -> Vec<String> {
    vec!["product_name: name".to_string()]
}

#[tokio::test]
async fn should_create_an_onchain_invoice_successfully() {
    let storage = Arc::new(Storage::new(true).await.expect("storage creation failed"));

    storage
        .init(&[
            KadeInvoiceService::CREATE_TABLE,
            KadeWalletService::CREATE_TABLE,
            KadeInvoiceService::CREATE_CHILD_INDICES_TABLE,
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

    let invoice_service = KadeInvoiceService::new(storage);

    let invoice_req = NewInvoiceRequest {
        x_pub_key_id: new_wallet_res.x_pub_key_id.to_string(),
        chain: "Bitcoin".to_string(),
        network: "testnet".to_string(),
        currency_code: "SATS".to_string(),
        amount: "34000000".to_string(),
        description: "Create an invoice on Bitcoin test".to_string(),
        metadata: get_invoice_metadata(),
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
    assert_eq!(new_invoice_res.amount, "34000000");
    assert_eq!(
        new_invoice_res.description,
        "Create an invoice on Bitcoin test"
    );
    assert_eq!(new_invoice_res.metadata, get_invoice_metadata());
    assert_eq!(new_invoice_res.chain, "Bitcoin");
    assert_eq!(new_invoice_res.currency_code, "SATS");
    assert!(
        !new_invoice_res.address.is_empty(),
        "expect a non-empty invoice address"
    );
    let address = Address::from_str(&new_invoice_res.address).unwrap();
    assert!(address.is_valid_for_network(Network::Testnet));
    assert_eq!(new_invoice_res.status, "pending");
    assert!(new_invoice_res.created_at > 0)
}

#[tokio::test]
async fn should_create_an_offchain_invoice_successfully() {
    let storage = Arc::new(Storage::new(true).await.expect("storage creation failed"));

    storage
        .init(&[
            KadeInvoiceService::CREATE_TABLE,
            KadeWalletService::CREATE_TABLE,
            KadeInvoiceService::CREATE_CHILD_INDICES_TABLE,
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

    let invoice_service = KadeInvoiceService::new_test(storage);

    let invoice_req = NewInvoiceRequest {
        x_pub_key_id: new_wallet_res.x_pub_key_id.to_string(),
        chain: "Arkade".to_string(),
        network: "testnet".to_string(),
        currency_code: "SATS".to_string(),
        amount: "34000000".to_string(),
        description: "Create an invoice on Bitcoin test".to_string(),
        metadata: get_invoice_metadata(),
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
    assert_eq!(new_invoice_res.amount, "34000000");
    assert_eq!(
        new_invoice_res.description,
        "Create an invoice on Bitcoin test"
    );
    assert_eq!(new_invoice_res.metadata, get_invoice_metadata());
    assert_eq!(new_invoice_res.chain, "Arkade");
    assert_eq!(new_invoice_res.currency_code, "SATS");
    assert!(
        !new_invoice_res.address.is_empty(),
        "expect a non-empty invoice address"
    );

    let address = ArkAddress::from_str(&new_invoice_res.address).unwrap();

    assert!(!address.encode().is_empty());
    assert_eq!(new_invoice_res.status, "pending");
    assert!(new_invoice_res.created_at > 0)
}

#[tokio::test]
async fn should_fail_creating_an_invoice_with_unmatching_network_to_ark_network() {
    let storage = Arc::new(Storage::new(true).await.expect("storage creation failed"));

    storage
        .init(&[
            KadeInvoiceService::CREATE_TABLE,
            KadeWalletService::CREATE_TABLE,
            KadeInvoiceService::CREATE_CHILD_INDICES_TABLE,
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

    let invoice_service = KadeInvoiceService::new_test(storage);

    let invoice_req = NewInvoiceRequest {
        x_pub_key_id: new_wallet_res.x_pub_key_id.to_string(),
        chain: "Arkade".to_string(),
        network: "regtest".to_string(),
        currency_code: "SATS".to_string(),
        amount: "34000000".to_string(),
        description: "Create an invoice on Bitcoin test".to_string(),
        metadata: get_invoice_metadata(),
    };

    let grpc_req = Request::new(invoice_req);

    let new_invoice_res = invoice_service.create_invoice(grpc_req).await;

    assert!(new_invoice_res.is_err());
    let status = new_invoice_res.err().unwrap();
    assert_eq!(status.code(), tonic::Code::InvalidArgument);
    assert_eq!(
        status.message(),
        "Arkade server network testnet does not match invoice network regtest"
    )
}

#[tokio::test]
async fn should_create_new_onchain_payment_address_for_every_new_invoice_successfully() {
    let storage = Arc::new(Storage::new(true).await.expect("storage creation failed"));

    storage
        .init(&[
            KadeInvoiceService::CREATE_TABLE,
            KadeWalletService::CREATE_TABLE,
            KadeInvoiceService::CREATE_CHILD_INDICES_TABLE,
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

    let invoice_service = KadeInvoiceService::new(storage);

    let mut prev_address = "".to_string();
    let mut seen_addresses: HashSet<String> = HashSet::new();

    for _ in 0..16 {
        let invoice_req = NewInvoiceRequest {
            x_pub_key_id: new_wallet_res.x_pub_key_id.to_string(),
            chain: "Bitcoin".to_string(),
            network: "testnet".to_string(),
            currency_code: "SATS".to_string(),
            amount: "34000000".to_string(),
            description: "Create an invoice on Bitcoin test".to_string(),
            metadata: get_invoice_metadata(),
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
        assert_eq!(new_invoice_res.amount, "34000000");
        assert_eq!(
            new_invoice_res.description,
            "Create an invoice on Bitcoin test"
        );
        assert_eq!(new_invoice_res.chain, "Bitcoin");
        assert_eq!(new_invoice_res.currency_code, "SATS");
        assert!(
            !new_invoice_res.address.is_empty(),
            "expect a non-empty invoice address"
        );
        assert_ne!(
            new_invoice_res.address, prev_address,
            "expect the new invoice payment address to be different from the previous invoice payment address"
        );
        assert!(seen_addresses.insert(new_invoice_res.address.clone()));

        let address = Address::from_str(&new_invoice_res.address).unwrap();
        assert!(address.is_valid_for_network(Network::Testnet));

        assert_eq!(new_invoice_res.status, "pending");
        assert!(new_invoice_res.created_at > 0);
        prev_address = new_invoice_res.address;
    }
}

#[tokio::test]
async fn should_create_new_onchain_payment_address_for_every_new_invoice_from_different_wallets_successfully()
 {
    let wallets = vec![
        "tpubDDneEXG899zhkpQt6bqo7fmaSVVi7ErfjNSs82gmTKJHJM5dfzT6f4er8dqgt85z3TYZYzJ7FZeTzKSkX1KKs8ejtXGg4FudTA9TR55ntaF".to_string(),
        "tpubDD1zWV61pKrXhEDL98mbtigniPSEH554pFGJAmoZESF7U2MYBHBktChKvh22HUK5BeQbxd2g73emUsG499U28qEue6Qq5Nrig1NA9ZHFnS4".to_string(),
        "tpubDCuC7dUW3oqvoWWvJdJR8BeadLCCd7oawFrLjj5dmxNBfkPAfjnTmhEvGtwY1kbQnud32RgHaq2RbB65yJvpEZv4ywonKZ98WE424JcgcMR".to_string(),
        "tpubDCoPmg2g4ZxYktnZHU2qnv7NqXkukAZJUEFwBVYacsLK7STYTfEGrn27FBLqtzZgiqnDEmZKh2yeWbdDF3WFFP7qTKJLzKLWEFDauf5EZXk".to_string(),
        "tpubDCieR5ToJTzTzf1hvtLq4US92JWswcwgsh5HjZXomPgAm2Jf6XY88sWJzi5J9hNqSDtC38pH3PVHH9UFbWUVueukERSHTB1h5WQsXsuFJ7V".to_string(),
        "tpubDD9H9EhzmpQxT16fcr4dwp6YbiZ5qZSpqssTwQ3jDN7v4Ki5Z8Zf1KnDWZZM6ZCW6t2XV8AgT6yaeWkaiouMkWpwp3SB8HaU98qu5kL34fP".to_string(),
        "tpubDCLvYStUxmVszzEqxFs4UQAnJUyd2yEEas8N9kaBbddPuXCbe6xQyoEy1b9qeTfrF4GdjYVgXkMLuUi4QbBs5qhX7NBFyJjgi4f2unGXQ1k".to_string(),
        "tpubDCPFEzrBZNz3P1ThSirxwxAj3BZZ6aPbkAwtSm3HdWBz1UMR4GLvXWiNVmiskke627g3k6KgNYk15kERsg8xBxX4Z9q9tnumNf4D4bjFY3V".to_string(),
        "tpubDCLuBUeExyGUCps4k7Hp27azTm4vzGhQaJpXFbxjeCU5hXaoyexu3KRbheXEzLUpEPheFi1D1CCJF874UguGciwccFkBLN1vF4KN8jpXYxu".to_string()
    ];

    let storage = Arc::new(Storage::new(true).await.expect("storage creation failed"));

    storage
        .init(&[
            KadeInvoiceService::CREATE_TABLE,
            KadeWalletService::CREATE_TABLE,
            KadeInvoiceService::CREATE_CHILD_INDICES_TABLE,
        ])
        .await
        .expect("storage initialization failed");

    let wallet_service = KadeWalletService::new(storage.clone());
    let invoice_service = KadeInvoiceService::new(storage);

    let mut prev_address = "".to_string();
    let mut seen_addresses: HashSet<String> = HashSet::new();

    for x_pub_key in wallets {
        let wallet_req = NewWalletRequest {
            x_pub_key: x_pub_key.to_string(),
        };
        let grpc_req = Request::new(wallet_req);
        let new_wallet_res = wallet_service
            .create_wallet(grpc_req)
            .await
            .expect("failed to create wallet")
            .into_inner();

        for _ in 0..16 {
            let invoice_req = NewInvoiceRequest {
                x_pub_key_id: new_wallet_res.x_pub_key_id.to_string(),
                chain: "Bitcoin".to_string(),
                network: "testnet".to_string(),
                currency_code: "SATS".to_string(),
                amount: "34000000".to_string(),
                description: "Create an invoice on Bitcoin test".to_string(),
                metadata: get_invoice_metadata(),
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
            assert_eq!(new_invoice_res.amount, "34000000");
            assert_eq!(
                new_invoice_res.description,
                "Create an invoice on Bitcoin test"
            );
            assert_eq!(new_invoice_res.chain, "Bitcoin");
            assert_eq!(new_invoice_res.currency_code, "SATS");
            assert!(
                !new_invoice_res.address.is_empty(),
                "expect a non-empty invoice address"
            );
            assert_ne!(
                new_invoice_res.address, prev_address,
                "expect the new invoice payment address to be different from the previous invoice payment address"
            );

            assert!(seen_addresses.insert(new_invoice_res.address.clone()));
            assert_eq!(new_invoice_res.status, "pending");
            assert!(new_invoice_res.created_at > 0);

            let address = Address::from_str(&new_invoice_res.address).unwrap();

            assert!(address.is_valid_for_network(Network::Testnet));

            prev_address = new_invoice_res.address;
        }
    }
}

#[tokio::test]
async fn should_atomically_create_concurrent_invoices_in_the_same_wallet_successfully() {
    let storage = Arc::new(Storage::new(true).await.expect("storage creation failed"));

    storage
        .init(&[
            KadeInvoiceService::CREATE_TABLE,
            KadeInvoiceService::CREATE_CHILD_INDICES_TABLE,
            KadeWalletService::CREATE_TABLE,
        ])
        .await
        .expect("storage initialization failed");

    let wallet_service = KadeWalletService::new(storage.clone());

    let invoice_service = KadeInvoiceService::new(storage);

    let wallet_req = NewWalletRequest {
        x_pub_key: "tpubDD1zWV61pKrXhEDL98mbtigniPSEH554pFGJAmoZESF7U2MYBHBktChKvh22HUK5BeQbxd2g73emUsG499U28qEue6Qq5Nrig1NA9ZHFnS4".to_string(),
    };
    let grpc_req = Request::new(wallet_req);
    let new_wallet_res = wallet_service
        .create_wallet(grpc_req)
        .await
        .expect("failed to create wallet")
        .into_inner();

    let invoice_req = NewInvoiceRequest {
        x_pub_key_id: new_wallet_res.x_pub_key_id.to_string(),
        chain: "Bitcoin".to_string(),
        network: "testnet".to_string(),
        currency_code: "SATS".to_string(),
        amount: "34000000".to_string(),
        description: "Create an invoice on Bitcoin test".to_string(),
        metadata: get_invoice_metadata(),
    };

    let grpc_req = Request::new(invoice_req.clone());
    let grpc_req_1 = Request::new(invoice_req.clone());
    let grpc_req_2 = Request::new(invoice_req.clone());
    let grpc_req_3 = Request::new(invoice_req.clone());

    let (result, result1, result2, result3) = tokio::join!(
        invoice_service.create_invoice(grpc_req),
        invoice_service.create_invoice(grpc_req_1),
        invoice_service.create_invoice(grpc_req_2),
        invoice_service.create_invoice(grpc_req_3)
    );

    assert!(result.is_ok());
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());

    let invoice_res = result.unwrap().into_inner();
    let invoice_res1 = result1.unwrap().into_inner();
    let invoice_res2 = result2.unwrap().into_inner();
    let invoice_res3 = result3.unwrap().into_inner();

    assert_ne!(invoice_res.address, invoice_res1.address);
    assert_ne!(invoice_res.address, invoice_res2.address);
    assert_ne!(invoice_res.address, invoice_res3.address);
    assert_ne!(invoice_res1.address, invoice_res2.address);
    assert_ne!(invoice_res1.address, invoice_res3.address);
    assert_ne!(invoice_res2.address, invoice_res3.address);
}
#[tokio::test]
async fn should_clean_up_unused_child_key_indices_after_failure() {
    let storage = Arc::new(Storage::new(true).await.expect("storage creation failed"));

    storage
        .init(&[
            KadeInvoiceService::CREATE_TABLE,
            KadeInvoiceService::CREATE_CHILD_INDICES_TABLE,
            KadeWalletService::CREATE_TABLE,
        ])
        .await
        .expect("storage initialization failed");

    let wallet_service = KadeWalletService::new(storage.clone());
    let invoice_service = KadeInvoiceService::new(storage.clone());

    let wallet_req = NewWalletRequest {
        x_pub_key: "tpubDD1zWV61pKrXhEDL98mbtigniPSEH554pFGJAmoZESF7U2MYBHBktChKvh22HUK5BeQbxd2g73emUsG499U28qEue6Qq5Nrig1NA9ZHFnS4".to_string(),
    };
    let grpc_req = Request::new(wallet_req);
    let new_wallet_res = wallet_service
        .create_wallet(grpc_req)
        .await
        .expect("failed to create wallet")
        .into_inner();

    let invoice_req = NewInvoiceRequest {
        x_pub_key_id: new_wallet_res.x_pub_key_id.to_string(),
        chain: "Bitcoin".to_string(),
        network: "testnet".to_string(),
        currency_code: "SATS".to_string(),
        amount: "34000000SATS".to_string(),
        description: "Create an invoice on Bitcoin test".to_string(),
        metadata: get_invoice_metadata(),
    };

    let grpc_req = Request::new(invoice_req.clone());
    let grpc_req_1 = Request::new(invoice_req.clone());
    let grpc_req_2 = Request::new(invoice_req.clone());
    let grpc_req_3 = Request::new(invoice_req.clone());

    let (result, result1, result2, result3) = tokio::join!(
        invoice_service.create_invoice(grpc_req),
        invoice_service.create_invoice(grpc_req_1),
        invoice_service.create_invoice(grpc_req_2),
        invoice_service.create_invoice(grpc_req_3)
    );

    assert!(result.is_err());
    assert!(result1.is_err());
    assert!(result2.is_err());
    assert!(result3.is_err());

    // Assert that every call cleaned up it's failed index
    let x_pub_key_id =
        Uuid::from_str(new_wallet_res.x_pub_key_id.as_str()).expect("failed to parse x_pub_key_id");
    let indices = storage
        .query(
            KadeInvoiceService::SELECT_CHILD_INDICES_BY_WALLET,
            &[&x_pub_key_id],
        )
        .await
        .expect("query failed");
    assert_eq!(indices.len(), 0);
}

#[tokio::test]
async fn should_fetch_invoices_successfully() {
    let storage = Arc::new(Storage::new(true).await.expect("storage creation failed"));
    storage
        .init(&[
            KadeInvoiceService::CREATE_TABLE,
            KadeInvoiceService::CREATE_CHILD_INDICES_TABLE,
            KadeWalletService::CREATE_TABLE,
        ])
        .await
        .expect("storage initialization failed");

    let invoice_service = KadeInvoiceService::new(storage.clone());
    let wallet_service = KadeWalletService::new(storage.clone());

    let wallet_req = NewWalletRequest {
        x_pub_key: "tpubDD1zWV61pKrXhEDL98mbtigniPSEH554pFGJAmoZESF7U2MYBHBktChKvh22HUK5BeQbxd2g73emUsG499U28qEue6Qq5Nrig1NA9ZHFnS4".to_string(),
    };
    let grpc_req = Request::new(wallet_req);
    let new_wallet_res = wallet_service
        .create_wallet(grpc_req)
        .await
        .expect("failed to create wallet")
        .into_inner();
    let invoice_req = NewInvoiceRequest {
        x_pub_key_id: new_wallet_res.x_pub_key_id.to_string(),
        chain: "Bitcoin".to_string(),
        network: "testnet".to_string(),
        currency_code: "SATS".to_string(),
        amount: "34000000".to_string(),
        description: "Create an invoice on Bitcoin test".to_string(),
        metadata: get_invoice_metadata(),
    };

    for _ in 0..10 {
        let grpc_req = Request::new(invoice_req.clone());
        invoice_service.create_invoice(grpc_req).await.unwrap();
    }

    let get_invoices_req = GetInvoicesRequest {
        x_pub_key_id: new_wallet_res.x_pub_key_id.to_string(),
    };
    let invoices_req = Request::new(get_invoices_req);
    let invoices_res = invoice_service
        .get_invoices(invoices_req)
        .await
        .expect("failed to fetch invoices");

    let invoices = invoices_res.into_inner().invoices;

    assert_eq!(invoices.len(), 10);

    let mut child_key_index = 0;
    invoices.iter().for_each(|invoice| {
        assert_eq!(
            invoice.x_pub_key_id,
            new_wallet_res.x_pub_key_id.to_string()
        );
        assert_eq!(invoice.child_key_index, child_key_index);
        child_key_index += 1
    })
}

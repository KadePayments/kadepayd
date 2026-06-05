use kadepayd::data::storage::Storage;
use kadepayd::invoice::NewInvoiceRequest;
use kadepayd::invoice::invoice_service_server::InvoiceService;
use kadepayd::services::invoice_service::KadeInvoiceService;
use std::sync::Arc;
use tonic::Request;

#[tokio::test]
async fn should_create_an_invoice_successfully() {
    let storage = Arc::new(Storage::new(true).await.expect("storage creation failed"));

    storage
        .init(&[KadeInvoiceService::CREATE_TABLE])
        .await
        .expect("storage initialization failed");

    let invoice_service = KadeInvoiceService::new(storage);

    let invoice_req = NewInvoiceRequest {
        pub_key_id: "c40f89d6-518f-4d9b-9c62-45c2cea7edc5".to_string(),
        network: "Arkade".to_string(),
        currency_code: "BTC".to_string(),
        amount: "0.0034".to_string(),
        description: "Create invoice on Arkade test".to_string(),
    };

    let grpc_req = Request::new(invoice_req);

    let new_invoice_res = invoice_service
        .create_invoice(grpc_req)
        .await
        .expect("failed to create new invoice")
        .into_inner();

    assert_eq!(
        new_invoice_res.pub_key_id,
        "c40f89d6-518f-4d9b-9c62-45c2cea7edc5"
    );
    assert_eq!(new_invoice_res.amount, "0.00340000");
    assert_eq!(new_invoice_res.description, "Create invoice on Arkade test");
    assert_eq!(new_invoice_res.network, "Arkade");
    assert_eq!(new_invoice_res.currency_code, "BTC");
    assert!(
        !new_invoice_res.address.is_empty(),
        "expect a non-empty invoice address"
    );
    assert_eq!(new_invoice_res.status, "pending");
    assert!(new_invoice_res.created_at > 0)
}

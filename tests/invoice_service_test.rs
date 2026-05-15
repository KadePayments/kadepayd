use kadepayd::data::storage::Storage;
use kadepayd::invoice::NewInvoiceRequest;
use kadepayd::invoice::invoice_service_server::InvoiceService;
use kadepayd::services::invoice_service::KadeInvoiceService;
use tonic::Request;

#[tokio::test]
async fn should_create_an_invoice_successfully() {
    let storage = Storage::new(true).await.unwrap();

    storage
        .init(&[KadeInvoiceService::CREATE_TABLE])
        .await
        .unwrap();

    let service = KadeInvoiceService::new(storage);

    let invoice_req = NewInvoiceRequest {
        network: "Arkade".to_string(),
        currency_code: "BTC".to_string(),
        amount: "0.0034".to_string(),
        description: "Create invoice on Arkade test".to_string(),
    };

    let grpc_req = Request::new(invoice_req);

    let new_invoice_res = service.create_invoice(grpc_req).await.unwrap().into_inner();

    assert_eq!(new_invoice_res.amount, "0.00340000");
    assert_eq!(new_invoice_res.description, "Create invoice on Arkade test");
    assert_eq!(new_invoice_res.network, "Arkade");
    assert_eq!(new_invoice_res.currency_code, "BTC");
    assert_eq!(new_invoice_res.address, "<ark1...>");
    assert_eq!(new_invoice_res.status, "pending");
    assert!(new_invoice_res.created_at > 0)
}

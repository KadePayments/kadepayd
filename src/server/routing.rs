use crate::data::{NewInvoiceQuery, NewInvoiceResponse, NewPaymentRequest};
use crate::invoice::invoice_service_client::InvoiceServiceClient;
use axum::extract::{Query, State};
use axum::http::Uri;
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::{get, post};
use axum::{Json, Router};
use std::str::FromStr;
use tonic::Status;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    pub invoice_client: InvoiceServiceClient<tonic::transport::Channel>,
}

pub async fn routes(url: String) -> Router {
    let uri: Uri = Uri::from_str(&url).unwrap();
    let grpc_channel = tonic::transport::Channel::builder(uri).connect_lazy();

    let invoice_client = InvoiceServiceClient::new(grpc_channel);
    let app_state = AppState { invoice_client };

    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    Router::new()
        .route("/pay/invoice", post(new_payment))
        .route("/pay/invoice", get(new_invoice_page))
        .with_state(app_state)
        .layer(cors)
}

async fn new_payment(
    State(state): State<AppState>,
    Json(info): Json<NewPaymentRequest>,
) -> impl IntoResponse {
    let mut invoice_client = state.invoice_client;
    let request = tonic::Request::new(info.to_invoice_request());
    let invoice = invoice_client
        .create_invoice(request)
        .await
        .expect("payment request creation failed")
        .into_inner();

    Json(NewInvoiceResponse::from_response(invoice))
}

async fn new_invoice_page(
    State(state): State<AppState>,
    Query(params): Query<NewInvoiceQuery>,
) -> Html<String> {
    let id = &params.id;
    let page = format!(
        "
        <!DOCTYPE html>
            <html>
                <head><title>KadePay</title></head>
                <body>
                    <h1>KadePay</h1>
                    <p>Invoice Id: {}</p>
                </body>
            </html>
    ",
        id
    );
    Html(page)
}

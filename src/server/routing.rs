use crate::core::svg_qr_code;
use crate::data::{NewInvoiceQuery, NewInvoiceResponse, NewPaymentRequest};
use crate::invoice::invoice_service_client::InvoiceServiceClient;
use axum::extract::{Query, State};
use axum::http::Uri;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, post};
use axum::{Json, Router};
use std::str::FromStr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

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
        .nest_service("/static", ServeDir::new("assets"))
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

    let qr_code = svg_qr_code("tb1pw2nqu22jj7qyvtaeje7tyzutgcs935lakt8g30pw27wjv57mfg0srxqc7r");

    let page = format!(
        "
        <!DOCTYPE html>
            <html>
                <head><title>KadePay</title></head>
                <body style=\"padding:0; margin:0; background:#F6FBF2; font-family:sans-serif;\">
                    <div style=\"display:flex; justify-content:center; height: 100vh; padding:0;\">
                        <div style=\"background:#EBEFE7; width:26em; height:fit-content; margin:auto; text-align:center; border-radius: 16px; border: 2px solid #717970;\">
                            <h1>KadePay</h1>
                            <ul style=\"display:flex; flex-direction: row; align-items: center; width:fit-content; gap:120px; padding:0; margin:auto; margin-bottom:20px;\">
                                <li style=\"display:inline-block; \"><p style=\"width:fit-content; font-size:1.3rem;\">2000 SATS</p></li>
                                <li style=\"display:inline-block; \"><p style=\"width:fit-content; font-size:1.3rem;\">Pending</p></li>
                            </ul>
                            <div>{}</div>
                            <div  style=\"background:#F6FBF2; margin: 20px; height: fit-content; padding:10px; border-radius:5px;\">
                                <p style=\"overflow-wrap:break-word; height: fit-content; border-radius:8px;\">tb1pw2nqu22jj7qyvtaeje7tyzutgcs935lakt8g30pw27wjv57mfg0srxqc7r</p>
                                <img src=\"/static/icons/copy.svg\"  style = \"color: #F9FAEF; background: #EBEFE7; padding: 12px; width: 24px; height: 24px; border-radius: 24px;\"/>
                            </div>
                        </div>
                    </div>
                </body>
            </html>
    ",
        qr_code,
    );
    Html(page)
}

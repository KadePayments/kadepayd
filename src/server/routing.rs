use crate::core::{format_amount, sats_from_str, sats_to_btc, svg_qr_code};
use crate::data::{NewInvoiceQuery, NewInvoiceResponse, NewPaymentRequest};
use crate::invoice::GetInvoiceRequest;
use crate::invoice::invoice_service_client::InvoiceServiceClient;
use axum::extract::{Query, State};
use axum::http::{Method, StatusCode, Uri};
use axum::response::{Html, IntoResponse};
use std::os::linux::raw::stat;

use crate::core::bitcoin::uri::encode_bitcoin_uri;
use crate::server::config::Config;
use crate::server::to_http_status;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::routing::{get, post};
use axum::{Json, Router};
use html_escape::encode_text_to_string;
use std::str::FromStr;
use tonic::Request;
use tonic::codegen::http::header::ACCESS_CONTROL_ALLOW_ORIGIN;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState {
    pub config: Config,
    pub invoice_client: InvoiceServiceClient<tonic::transport::Channel>,
}

pub async fn routes(config: &Config) -> Router {
    let uri: Uri = Uri::from_str(&config.api_url).unwrap();
    let grpc_channel = tonic::transport::Channel::builder(uri).connect_lazy();

    let invoice_client = InvoiceServiceClient::new(grpc_channel);
    let asset_dir = config.asset_dir.clone();
    let app_state = AppState {
        config: config.clone(),
        invoice_client,
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION, ACCESS_CONTROL_ALLOW_ORIGIN])
        .allow_origin(Any);

    Router::new()
        .route("/", get(heartbeat))
        .route("/pay/invoice", post(new_payment))
        .route("/pay/invoice", get(new_invoice_page))
        .nest_service("/static", ServeDir::new(&asset_dir))
        .with_state(app_state)
        .layer(cors)
}

async fn heartbeat() -> impl IntoResponse {
    (StatusCode::OK, "KadePay is breathing!")
}

async fn new_payment(
    State(state): State<AppState>,
    Json(info): Json<NewPaymentRequest>,
) -> impl IntoResponse {
    let mut invoice_client = state.invoice_client;
    let request = Request::new(info.to_invoice_request());
    let invoice = match invoice_client.create_invoice(request).await {
        Ok(invoice) => invoice.into_inner(),
        Err(status) => return to_http_status(status).into_response(),
    };

    Json(NewInvoiceResponse::from_response(invoice)).into_response()
}

async fn new_invoice_page(
    State(state): State<AppState>,
    Query(params): Query<NewInvoiceQuery>,
) -> impl IntoResponse {
    let invoice_req = GetInvoiceRequest { id: params.id };
    let invoice_grpc_req = Request::new(invoice_req);
    let mut invoice_client = state.invoice_client;
    let invoice = match invoice_client.get_invoice(invoice_grpc_req).await {
        Ok(invoice) => invoice.into_inner(),
        Err(status) => return to_http_status(status).into_response(),
    };

    let mut address = "".to_string();
    encode_text_to_string(invoice.address.as_str(), &mut address);

    let mut invoice_status = "".to_string();
    encode_text_to_string(invoice.status.to_uppercase(), &mut invoice_status);

    let mut amount = "".to_string();
    encode_text_to_string(
        match format_amount(invoice.amount.as_str(), invoice.currency_code.as_str()) {
            Ok(amount) => amount,
            Err(status) => return to_http_status(status).into_response(),
        },
        &mut amount,
    );

    let btc_amount = match sats_from_str(&invoice.amount) {
        Ok(amount) => amount,
        Err(status) => return to_http_status(status).into_response(),
    };

    let uri = encode_bitcoin_uri(
        &address,
        &btc_amount,
        &invoice.metadata["label"],
        &invoice.description,
        &invoice.network,
    )
    .to_uppercase();

    let qr_code = match svg_qr_code(uri.as_str(), state.config.asset_dir) {
        Ok(qr_code) => qr_code,
        Err(status) => return to_http_status(status).into_response(),
    };

    let page = format!(
        "
        <!DOCTYPE html>
            <html>
                <head>
                    <title>KadePay</title>
                    <style>
                        #address-container {{
                            background:#F6FBF2;
                            margin: 20px;
                            height: fit-content;
                            padding: 10px;
                            border-radius: 10px;
                        }}
                        #address-container:active {{
                            background: #c6d3ba;
                        }}
                    </style>
                </head>
                <body style=\"padding:0; margin:0; background:#F6FBF2; font-family:sans-serif;\">
                    <div style=\"display:flex; justify-content:center; height: 100vh; padding:0;\">
                        <div style=\"background:#EBEFE7; width:26em; height:fit-content; margin:auto; text-align:center; border-radius: 16px; border: 1px solid #717970;\">
                            <h1>KadePay</h1>
                            <ul style=\"display:flex; flex-direction: row; align-items: center; width:fit-content; gap:120px; padding:0; margin:auto; margin-bottom:20px;\">
                                <li style=\"display:inline-block; \"><p style=\"width:fit-content; font-size:1.3rem;\">{amount}</p></li>
                                <li style=\"display:inline-block; \"><p style=\"width:fit-content; font-size:1.3rem;\">{invoice_status}</p></li>
                            </ul>
                            <div style=\"border-radius:5px;\">{qr_code}</div>
                            <div  id=\"address-container\">
                                <p style=\"overflow-wrap:break-word; height: fit-content; border-radius:8px;\">{address}</p>
                                <img src=\"/static/icons/copy.svg\"  style = \"color: #F9FAEF; background: #EBEFE7; padding: 12px; width: 24px; height: 24px; border-radius: 24px;\"/>
                            </div>
                        </div>
                    </div>
                    <script type=\"text/javascript\">
                        const addressContainer = document.getElementById(\"address-container\");
                        addressContainer.addEventListener(
                            'click',
                            () => {{
                                const address = addressContainer.querySelector('p').textContent
                                navigator.clipboard.writeText(address)
                            }}
                        )
                    </script>
                </body>
            </html>
    "
    );
    Html(page).into_response()
}

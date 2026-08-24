use axum::http::StatusCode;
use tonic::{Code, Status};

pub mod config;
pub mod engine;
pub mod routing;

pub fn to_http_status(status: Status) -> StatusCode {
    match status.code() {
        Code::Ok => StatusCode::OK,                               // 200
        Code::Cancelled => StatusCode::BAD_REQUEST,               // 400 (or 499 client closed)
        Code::Unknown => StatusCode::INTERNAL_SERVER_ERROR,       // 500
        Code::InvalidArgument => StatusCode::BAD_REQUEST,         // 400
        Code::DeadlineExceeded => StatusCode::GATEWAY_TIMEOUT,    // 544
        Code::NotFound => StatusCode::NOT_FOUND,                  // 404
        Code::AlreadyExists => StatusCode::CONFLICT,              // 409
        Code::PermissionDenied => StatusCode::FORBIDDEN,          // 403
        Code::ResourceExhausted => StatusCode::TOO_MANY_REQUESTS, // 429
        Code::FailedPrecondition => StatusCode::BAD_REQUEST,      // 400
        Code::Aborted => StatusCode::CONFLICT,                    // 409
        Code::OutOfRange => StatusCode::BAD_REQUEST,              // 400
        Code::Unimplemented => StatusCode::NOT_IMPLEMENTED,       // 501
        Code::Internal => StatusCode::INTERNAL_SERVER_ERROR,      // 500
        Code::Unavailable => StatusCode::SERVICE_UNAVAILABLE,     // 503
        Code::DataLoss => StatusCode::INTERNAL_SERVER_ERROR,      // 500
        Code::Unauthenticated => StatusCode::UNAUTHORIZED,        // 401
    }
}

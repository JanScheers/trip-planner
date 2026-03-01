use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
}

impl ApiError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self { error: msg.into() }
    }
}

impl From<&str> for ApiError {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(self)).into_response()
    }
}

pub fn not_found(message: &str) -> Response {
    (StatusCode::NOT_FOUND, axum::Json(ApiError::new(message))).into_response()
}

pub fn bad_request(message: impl Into<String>) -> Response {
    (StatusCode::BAD_REQUEST, axum::Json(ApiError::new(message))).into_response()
}

pub fn internal(message: impl Into<String>) -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        axum::Json(ApiError::new(message)),
    )
        .into_response()
}

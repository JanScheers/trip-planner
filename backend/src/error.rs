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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    async fn body_json(resp: Response) -> serde_json::Value {
        let bytes = to_bytes(resp.into_body(), 4096).await.unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn not_found_returns_404_with_error_field() {
        let resp = not_found("day not found");
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let json = body_json(resp).await;
        assert_eq!(json["error"], "day not found");
    }

    #[tokio::test]
    async fn bad_request_returns_400_with_error_field() {
        let resp = bad_request("invalid input");
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let json = body_json(resp).await;
        assert_eq!(json["error"], "invalid input");
    }

    #[tokio::test]
    async fn internal_returns_500_with_error_field() {
        let resp = internal("something went wrong");
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let json = body_json(resp).await;
        assert_eq!(json["error"], "something went wrong");
    }

    #[tokio::test]
    async fn api_error_into_response_is_500() {
        let err = ApiError::new("oops");
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let json = body_json(resp).await;
        assert_eq!(json["error"], "oops");
    }

    #[test]
    fn api_error_new_stores_message() {
        let err = ApiError::new("test message");
        assert_eq!(err.error, "test message");
    }

    #[test]
    fn api_error_from_str() {
        let err = ApiError::from("from str");
        assert_eq!(err.error, "from str");
    }
}

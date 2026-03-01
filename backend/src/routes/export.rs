use std::sync::Arc;

use axum::extract::State;
use axum::http::header;
use axum::response::IntoResponse;

use crate::auth::AppState;
use crate::db;

pub async fn export_tsv(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let tsv = db::export_tsv(&state.pool).await;
    (
        [(header::CONTENT_TYPE, "text/tab-separated-values")],
        tsv,
    )
}

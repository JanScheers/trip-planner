pub mod accommodations;
pub mod cities;
pub mod checklist;
pub mod days;
pub mod export;
pub mod tips;
pub mod upload;

use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post, put};
use axum::Router;

use crate::auth::AppState;

// Must be at least as large as upload::MAX_FILE_SIZE plus multipart overhead.
const UPLOAD_BODY_LIMIT: usize = 101 * 1024 * 1024; // 101 MiB

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/days", get(days::get_days).post(days::create_day))
        .route(
            "/api/days/{id}",
            get(days::get_day)
                .put(days::update_day)
                .delete(days::delete_day),
        )
        .route(
            "/api/cities",
            get(cities::get_cities).post(cities::create_city),
        )
        .route(
            "/api/cities/{key}",
            get(cities::get_city)
                .put(cities::update_city)
                .delete(cities::delete_city),
        )
        .route(
            "/api/accommodations",
            get(accommodations::get_accommodations).post(accommodations::create_accommodation),
        )
        .route(
            "/api/accommodations/{key}",
            get(accommodations::get_accommodation)
                .put(accommodations::update_accommodation)
                .delete(accommodations::delete_accommodation),
        )
        .route(
            "/api/checklist/editors",
            get(checklist::get_editors),
        )
        .route(
            "/api/checklist/items",
            get(checklist::get_items).post(checklist::create_item),
        )
        .route(
            "/api/checklist/items/{id}",
            put(checklist::update_item).delete(checklist::delete_item),
        )
        .route(
            "/api/checklist/checks",
            get(checklist::get_checks).put(checklist::put_check),
        )
        .route("/api/tips", get(tips::get_tips).put(tips::update_tips))
        .route("/api/export", get(export::export_tsv))
        .route(
            "/api/upload",
            post(upload::upload_image).layer(DefaultBodyLimit::max(UPLOAD_BODY_LIMIT)),
        )
}

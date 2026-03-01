use std::io::Write;

use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

pub async fn upload_image(mut multipart: Multipart) -> impl IntoResponse {
    while let Ok(Some(field)) = multipart.next_field().await {
        let filename = field
            .file_name()
            .unwrap_or("upload.jpg")
            .to_string();

        let ext = filename.rsplit('.').next().unwrap_or("jpg").to_string();
        let new_filename = format!("{}.{}", Uuid::new_v4(), ext);
        let filepath = format!("static/{}", new_filename);

        let data = match field.bytes().await {
            Ok(b) => b,
            Err(e) => {
                return (StatusCode::BAD_REQUEST, Json(e.to_string())).into_response();
            }
        };

        let mut file = match std::fs::File::create(&filepath) {
            Ok(f) => f,
            Err(e) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response();
            }
        };

        if let Err(e) = file.write_all(&data) {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response();
        }

        return Json(crate::models::UploadResponse {
            url: format!("/static/{}", new_filename),
        })
        .into_response();
    }

    (StatusCode::BAD_REQUEST, Json("No file provided")).into_response()
}

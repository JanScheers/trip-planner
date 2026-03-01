use std::io::Write;

use axum::extract::{Multipart, State};
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;
use uuid::Uuid;

use crate::auth::{AppState, RequireEditor};
use crate::error::{bad_request, internal};
use crate::models::UploadResponse;

const MAX_FILE_SIZE: usize = 5 * 1024 * 1024; // 5 MiB

fn allowed_image_ext(content_type: Option<&str>, bytes: &[u8]) -> Option<&'static str> {
    let ext = match content_type {
        Some("image/jpeg") | Some("image/jpg") => Some("jpg"),
        Some("image/png") => Some("png"),
        Some("image/gif") => Some("gif"),
        Some("image/webp") => Some("webp"),
        _ => {
            if bytes.len() >= 3 && bytes[0] == 0xFF && bytes[1] == 0xD8 && bytes[2] == 0xFF {
                Some("jpg")
            } else if bytes.len() >= 8
                && bytes[0..4] == [0x89, 0x50, 0x4E, 0x47]
                && bytes[4..8] == [0x0D, 0x0A, 0x1A, 0x0A]
            {
                Some("png")
            } else if bytes.len() >= 6
                && (bytes[0..6] == *b"GIF87a" || bytes[0..6] == *b"GIF89a")
            {
                Some("gif")
            } else if bytes.len() >= 12
                && bytes[8..12] == *b"WEBP"
            {
                Some("webp")
            } else {
                None
            }
        }
    };
    ext
}

pub async fn upload_image(
    _editor: RequireEditor,
    State(_state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    while let Ok(Some(field)) = multipart.next_field().await {
        let content_type = field.content_type().map(|ct| ct.to_string());
        let data = match field.bytes().await {
            Ok(b) => b,
            Err(e) => return bad_request(e.to_string()),
        };

        if data.len() > MAX_FILE_SIZE {
            return bad_request(format!("File too large (max {} MiB)", MAX_FILE_SIZE / (1024 * 1024)));
        }

        let ext = match allowed_image_ext(content_type.as_deref(), &data) {
            Some(ext) => ext,
            None => return bad_request("Invalid image type. Use JPEG, PNG, GIF, or WebP."),
        };

        let new_filename = format!("{}.{}", Uuid::new_v4(), ext);
        let filepath = format!("static/{}", new_filename);

        let mut file = match std::fs::File::create(&filepath) {
            Ok(f) => f,
            Err(e) => return internal(e.to_string()),
        };

        if let Err(e) = file.write_all(&data) {
            let _ = std::fs::remove_file(&filepath);
            return internal(e.to_string());
        }

        return Json(UploadResponse {
            url: format!("/static/{}", new_filename),
        })
        .into_response();
    }

    bad_request("No file provided")
}

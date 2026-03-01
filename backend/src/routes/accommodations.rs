use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

use crate::auth::{AppState, RequireEditor};
use crate::error::{bad_request, internal, not_found};
use crate::models::*;

pub async fn get_accommodations(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let accs: Vec<Accommodation> = sqlx::query_as("SELECT * FROM accommodations ORDER BY key")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();
    Json(accs)
}

pub async fn get_accommodation(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Accommodation>("SELECT * FROM accommodations WHERE key = ?")
        .bind(&key)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(Some(acc)) => Json(acc).into_response(),
        Ok(None) => not_found("Accommodation not found"),
        Err(e) => internal(e.to_string()),
    }
}

pub async fn create_accommodation(
    _editor: RequireEditor,
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateAccommodation>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Accommodation>(
        "INSERT INTO accommodations (key, name, link, notes) VALUES (?, ?, '', '') RETURNING *",
    )
    .bind(&body.key)
    .bind(&body.name)
    .fetch_one(&state.pool)
    .await
    {
        Ok(acc) => (axum::http::StatusCode::CREATED, Json(acc)).into_response(),
        Err(e) => bad_request(e.to_string()),
    }
}

pub async fn update_accommodation(
    _editor: RequireEditor,
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
    Json(body): Json<UpdateAccommodation>,
) -> impl IntoResponse {
    let existing =
        sqlx::query_as::<_, Accommodation>("SELECT * FROM accommodations WHERE key = ?")
            .bind(&key)
            .fetch_optional(&state.pool)
            .await;

    let existing = match existing {
        Ok(Some(a)) => a,
        Ok(None) => return not_found("Accommodation not found"),
        Err(e) => return internal(e.to_string()),
    };

    let name = body.name.as_deref().unwrap_or(&existing.name);
    let link = body.link.as_deref().unwrap_or(&existing.link);
    let notes = body.notes.as_deref().unwrap_or(&existing.notes);
    let emoji = body.emoji.as_ref().or(existing.emoji.as_ref());
    let hero_image = body.hero_image.as_ref().or(existing.hero_image.as_ref());

    match sqlx::query_as::<_, Accommodation>(
        "UPDATE accommodations SET name=?, link=?, notes=?, emoji=?, hero_image=? WHERE key=? RETURNING *",
    )
    .bind(name)
    .bind(link)
    .bind(notes)
    .bind(emoji)
    .bind(hero_image)
    .bind(&key)
    .fetch_one(&state.pool)
    .await
    {
        Ok(acc) => Json(acc).into_response(),
        Err(e) => internal(e.to_string()),
    }
}

pub async fn delete_accommodation(
    _editor: RequireEditor,
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM accommodations WHERE key = ?")
        .bind(&key)
        .execute(&state.pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => Json("Deleted").into_response(),
        Ok(_) => not_found("Accommodation not found"),
        Err(e) => internal(e.to_string()),
    }
}

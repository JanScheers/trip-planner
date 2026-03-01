use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::auth::AppState;
use crate::models::*;

pub async fn get_cities(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let cities: Vec<City> = sqlx::query_as("SELECT * FROM cities ORDER BY key")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();
    Json(cities)
}

pub async fn get_city(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, City>("SELECT * FROM cities WHERE key = ?")
        .bind(&key)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(Some(city)) => Json(serde_json::to_value(city).unwrap()).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json("City not found")).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

pub async fn update_city(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
    Json(body): Json<UpdateCity>,
) -> impl IntoResponse {
    let existing = sqlx::query_as::<_, City>("SELECT * FROM cities WHERE key = ?")
        .bind(&key)
        .fetch_optional(&state.pool)
        .await;

    let existing = match existing {
        Ok(Some(c)) => c,
        Ok(None) => return (StatusCode::NOT_FOUND, Json("City not found")).into_response(),
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response()
        }
    };

    let name = body.name.as_deref().unwrap_or(&existing.name);
    let chinese_name = body
        .chinese_name
        .as_deref()
        .unwrap_or(&existing.chinese_name);
    let notes = body.notes.as_deref().unwrap_or(&existing.notes);
    let emoji = body.emoji.as_ref().or(existing.emoji.as_ref());
    let hero_image = body.hero_image.as_ref().or(existing.hero_image.as_ref());

    match sqlx::query_as::<_, City>(
        "UPDATE cities SET name=?, chinese_name=?, notes=?, emoji=?, hero_image=? WHERE key=? RETURNING *",
    )
    .bind(name)
    .bind(chinese_name)
    .bind(notes)
    .bind(emoji)
    .bind(hero_image)
    .bind(&key)
    .fetch_one(&state.pool)
    .await
    {
        Ok(city) => Json(serde_json::to_value(city).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

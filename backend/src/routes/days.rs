use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::auth::AppState;
use crate::models::*;

pub async fn get_days(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let days: Vec<Day> = sqlx::query_as("SELECT * FROM days ORDER BY date")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();
    Json(days)
}

pub async fn get_day(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Day>("SELECT * FROM days WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(Some(day)) => Json(serde_json::to_value(day).unwrap()).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json("Day not found")).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

pub async fn create_day(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateDay>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Day>(
        "INSERT INTO days (date, city_key, accommodation_key, notes) VALUES (?, ?, ?, '') RETURNING *",
    )
    .bind(&body.date)
    .bind(&body.city_key)
    .bind(&body.accommodation_key)
    .fetch_one(&state.pool)
    .await
    {
        Ok(day) => (StatusCode::CREATED, Json(serde_json::to_value(day).unwrap())).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, Json(e.to_string())).into_response(),
    }
}

pub async fn update_day(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateDay>,
) -> impl IntoResponse {
    let existing = sqlx::query_as::<_, Day>("SELECT * FROM days WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.pool)
        .await;

    let existing = match existing {
        Ok(Some(d)) => d,
        Ok(None) => return (StatusCode::NOT_FOUND, Json("Day not found")).into_response(),
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response()
        }
    };

    let date = body.date.as_deref().unwrap_or(&existing.date);
    let city_key = body.city_key.as_deref().unwrap_or(&existing.city_key);
    let accommodation_key = body
        .accommodation_key
        .as_ref()
        .or(existing.accommodation_key.as_ref());
    let notes = body.notes.as_deref().unwrap_or(&existing.notes);
    let emoji = body.emoji.as_ref().or(existing.emoji.as_ref());
    let hero_image = body.hero_image.as_ref().or(existing.hero_image.as_ref());

    match sqlx::query_as::<_, Day>(
        "UPDATE days SET date=?, city_key=?, accommodation_key=?, notes=?, emoji=?, hero_image=? WHERE id=? RETURNING *",
    )
    .bind(date)
    .bind(city_key)
    .bind(accommodation_key)
    .bind(notes)
    .bind(emoji)
    .bind(hero_image)
    .bind(id)
    .fetch_one(&state.pool)
    .await
    {
        Ok(day) => Json(serde_json::to_value(day).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

pub async fn delete_day(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM days WHERE id = ?")
        .bind(id)
        .execute(&state.pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => Json("Deleted").into_response(),
        Ok(_) => (StatusCode::NOT_FOUND, Json("Day not found")).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response(),
    }
}

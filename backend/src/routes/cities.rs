use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

use crate::auth::{AppState, RequireEditor};
use crate::error::{bad_request, internal, not_found};
use crate::models::*;

fn slugify(s: &str) -> String {
    let s = s.trim().to_lowercase();
    let s: String = s
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c
            } else if c == ' ' || c == '-' {
                '-'
            } else {
                '\0'
            }
        })
        .collect();
    let s: String = s
        .split('-')
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    if s.is_empty() {
        "unnamed".to_string()
    } else {
        s
    }
}

pub async fn get_cities(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    // Order by first day date for each city (from days table); cities with no days last, then by key.
    let cities: Vec<City> = sqlx::query_as(
        "SELECT c.key, c.name, c.chinese_name, c.tagline, c.description, c.emoji, c.hero_image, c.lat, c.lng \
         FROM cities c \
         LEFT JOIN (SELECT city_key, MIN(date) AS first_date FROM days GROUP BY city_key) d ON c.key = d.city_key \
         ORDER BY d.first_date IS NULL, d.first_date ASC, c.key ASC",
    )
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
        Ok(Some(city)) => Json(city).into_response(),
        Ok(None) => not_found("City not found"),
        Err(e) => internal(e.to_string()),
    }
}

pub async fn update_city(
    _editor: RequireEditor,
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
        Ok(None) => return not_found("City not found"),
        Err(e) => return internal(e.to_string()),
    };

    let name = body.name.as_deref().unwrap_or(&existing.name);
    let chinese_name = body.chinese_name.as_deref().unwrap_or(&existing.chinese_name);
    let tagline = body.tagline.as_deref().unwrap_or(&existing.tagline);
    let description = body.description.as_deref().unwrap_or(&existing.description);
    let emoji = body.emoji.as_ref().or(existing.emoji.as_ref());
    let hero_image = body.hero_image.as_ref().or(existing.hero_image.as_ref());
    let lat = body.lat.or(existing.lat);
    let lng = body.lng.or(existing.lng);

    match sqlx::query_as::<_, City>(
        "UPDATE cities \
         SET name=?, chinese_name=?, tagline=?, description=?, emoji=?, hero_image=?, lat=?, lng=? \
         WHERE key=? RETURNING *",
    )
    .bind(name)
    .bind(chinese_name)
    .bind(tagline)
    .bind(description)
    .bind(emoji)
    .bind(hero_image)
    .bind(lat)
    .bind(lng)
    .bind(&key)
    .fetch_one(&state.pool)
    .await
    {
        Ok(city) => Json(city).into_response(),
        Err(e) => internal(e.to_string()),
    }
}

pub async fn create_city(
    _editor: RequireEditor,
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateCity>,
) -> impl IntoResponse {
    let chinese_name = body.chinese_name.as_deref().unwrap_or("");
    let tagline = body.tagline.as_deref().unwrap_or("");
    let base_key = slugify(&body.name);
    let mut key = base_key.clone();
    let mut n = 2;
    loop {
        let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM cities WHERE key = ?")
            .bind(&key)
            .fetch_one(&state.pool)
            .await
            .unwrap_or((0,));
        if exists.0 == 0 {
            break;
        }
        key = format!("{}-{}", base_key, n);
        n += 1;
    }
    match sqlx::query_as::<_, City>(
        "INSERT INTO cities (key, name, chinese_name, tagline, description, lat, lng) VALUES (?, ?, ?, ?, '', ?, ?) RETURNING *",
    )
    .bind(&key)
    .bind(&body.name)
    .bind(chinese_name)
    .bind(tagline)
    .bind(body.lat)
    .bind(body.lng)
    .fetch_one(&state.pool)
    .await
    {
        Ok(city) => (axum::http::StatusCode::CREATED, Json(city)).into_response(),
        Err(e) => bad_request(e.to_string()),
    }
}

pub async fn delete_city(
    _editor: RequireEditor,
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> impl IntoResponse {
    let day_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM days WHERE city_key = ?")
        .bind(&key)
        .fetch_one(&state.pool)
        .await
        .unwrap_or((0,));

    if day_count.0 > 0 {
        let city = sqlx::query_as::<_, City>("SELECT * FROM cities WHERE key = ?")
            .bind(&key)
            .fetch_optional(&state.pool)
            .await
            .ok()
            .flatten();
        let name = city.as_ref().map(|c| c.name.as_str()).unwrap_or(&key);
        return bad_request(format!(
            "Cannot delete: {} days are in {}. Reassign or delete those days first.",
            day_count.0, name
        ));
    }

    match sqlx::query("DELETE FROM cities WHERE key = ?")
        .bind(&key)
        .execute(&state.pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => Json("Deleted").into_response(),
        Ok(_) => not_found("City not found"),
        Err(e) => internal(e.to_string()),
    }
}

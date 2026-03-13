use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;

use crate::auth::{AppState, RequireEditor};
use crate::error::{bad_request, internal, not_found};
use crate::models::*;

pub async fn get_editors(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let editors: Vec<ChecklistEditor> = state
        .editor_emails
        .iter()
        .map(|email| ChecklistEditor {
            email: email.clone(),
        })
        .collect();
    Json(editors)
}

pub async fn get_items(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let items: Vec<ChecklistItem> = sqlx::query_as(
        "SELECT id, label, sort_order FROM checklist_items ORDER BY sort_order ASC, id ASC",
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();
    Json(items)
}

pub async fn create_item(
    _editor: RequireEditor,
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateChecklistItem>,
) -> impl IntoResponse {
    let sort_order = body.sort_order.unwrap_or(0);
    match sqlx::query_as::<_, ChecklistItem>(
        "INSERT INTO checklist_items (label, sort_order) VALUES (?, ?) RETURNING *",
    )
    .bind(&body.label)
    .bind(sort_order)
    .fetch_one(&state.pool)
    .await
    {
        Ok(item) => (axum::http::StatusCode::CREATED, Json(item)).into_response(),
        Err(e) => bad_request(e.to_string()),
    }
}

pub async fn update_item(
    _editor: RequireEditor,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(body): Json<UpdateChecklistItem>,
) -> impl IntoResponse {
    let existing = sqlx::query_as::<_, ChecklistItem>("SELECT * FROM checklist_items WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.pool)
        .await;

    let existing = match existing {
        Ok(Some(i)) => i,
        Ok(None) => return not_found("Checklist item not found"),
        Err(e) => return internal(e.to_string()),
    };

    let label = body.label.as_deref().unwrap_or(&existing.label);
    let sort_order = body.sort_order.unwrap_or(existing.sort_order);

    match sqlx::query_as::<_, ChecklistItem>(
        "UPDATE checklist_items SET label=?, sort_order=? WHERE id=? RETURNING *",
    )
    .bind(label)
    .bind(sort_order)
    .bind(id)
    .fetch_one(&state.pool)
    .await
    {
        Ok(item) => Json(item).into_response(),
        Err(e) => internal(e.to_string()),
    }
}

pub async fn delete_item(
    _editor: RequireEditor,
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM checklist_items WHERE id = ?")
        .bind(id)
        .execute(&state.pool)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => Json("Deleted").into_response(),
        Ok(_) => not_found("Checklist item not found"),
        Err(e) => internal(e.to_string()),
    }
}

pub async fn get_checks(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let rows: Vec<(i64, String, i64)> = sqlx::query_as(
        "SELECT item_id, editor_email, checked FROM checklist_checks",
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    let mut map: HashMap<String, bool> = HashMap::new();
    for (item_id, editor_email, checked) in rows {
        map.insert(format!("{}:{}", item_id, editor_email), checked != 0);
    }
    Json(map)
}

pub async fn put_check(
    RequireEditor(user): RequireEditor,
    State(state): State<Arc<AppState>>,
    Json(body): Json<ToggleCheckBody>,
) -> impl IntoResponse {
    let checked = if body.checked { 1 } else { 0 };

    // Verify item exists
    let exists: Option<(i64,)> = sqlx::query_as("SELECT id FROM checklist_items WHERE id = ?")
        .bind(body.item_id)
        .fetch_optional(&state.pool)
        .await
        .unwrap_or(None);

    if exists.is_none() {
        return not_found("Checklist item not found");
    }

    // Ensure user is in editor list (they are, since RequireEditor passed)
    match sqlx::query(
        "INSERT INTO checklist_checks (item_id, editor_email, checked) VALUES (?, ?, ?)
         ON CONFLICT(item_id, editor_email) DO UPDATE SET checked = excluded.checked",
    )
    .bind(body.item_id)
    .bind(&user.email)
    .bind(checked)
    .execute(&state.pool)
    .await
    {
        Ok(_) => Json(serde_json::json!({
            "item_id": body.item_id,
            "editor_email": user.email,
            "checked": body.checked
        }))
        .into_response(),
        Err(e) => internal(e.to_string()),
    }
}

use std::sync::Arc;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;

use crate::auth::{AppState, RequireEditor};
use crate::error::internal;
use crate::models::*;

const TIPS_KEY: &str = "main";

pub async fn get_tips(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match sqlx::query_as::<_, Tips>("SELECT key, content FROM tips WHERE key = ?")
        .bind(TIPS_KEY)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(Some(tips)) => Json(tips).into_response(),
        Ok(None) => Json(Tips {
            key: TIPS_KEY.to_string(),
            content: String::new(),
        })
        .into_response(),
        Err(e) => internal(e.to_string()),
    }
}

pub async fn update_tips(
    _editor: RequireEditor,
    State(state): State<Arc<AppState>>,
    Json(body): Json<UpdateTips>,
) -> impl IntoResponse {
    let content = body
        .content
        .as_deref()
        .unwrap_or("")
        .to_string();

    let existing = sqlx::query_as::<_, Tips>("SELECT key, content FROM tips WHERE key = ?")
        .bind(TIPS_KEY)
        .fetch_optional(&state.pool)
        .await;

    match existing {
        Ok(Some(_)) => {
            match sqlx::query_as::<_, Tips>(
                "UPDATE tips SET content = ? WHERE key = ? RETURNING key, content",
            )
            .bind(&content)
            .bind(TIPS_KEY)
            .fetch_one(&state.pool)
            .await
            {
                Ok(tips) => Json(tips).into_response(),
                Err(e) => internal(e.to_string()),
            }
        }
        Ok(None) => {
            match sqlx::query_as::<_, Tips>(
                "INSERT INTO tips (key, content) VALUES (?, ?) RETURNING key, content",
            )
            .bind(TIPS_KEY)
            .bind(&content)
            .fetch_one(&state.pool)
            .await
            {
                Ok(tips) => Json(tips).into_response(),
                Err(e) => internal(e.to_string()),
            }
        }
        Err(e) => internal(e.to_string()),
    }
}

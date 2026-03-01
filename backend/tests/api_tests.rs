use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use cookie::Cookie;
use axum_test::TestServer;
use sqlx::SqlitePool;

use backend::auth::AppState;
use backend::db;
use backend::models::AuthState;
use backend::routes;

const TEST_EDITOR_SESSION: &str = "test-editor-session";

fn editor_cookie() -> Cookie<'static> {
    Cookie::build(("session", TEST_EDITOR_SESSION)).path("/").build().into_owned()
}

async fn setup_pool() -> SqlitePool {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory pool");

    db::run_migrations(&pool).await;

    sqlx::query("INSERT OR IGNORE INTO cities (key, name, chinese_name) VALUES ('beijing', 'Beijing', '北京')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT OR IGNORE INTO cities (key, name, chinese_name) VALUES ('xian', 'Xi''an', '西安')")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT OR IGNORE INTO accommodations (key, name, link, notes) VALUES ('hotel-a', 'Hotel Alpha', '', '')")
        .execute(&pool)
        .await
        .unwrap();

    pool
}

async fn setup_server(pool: SqlitePool) -> TestServer {
    let mut sessions = HashMap::new();
    sessions.insert(
        TEST_EDITOR_SESSION.to_string(),
        AuthState {
            email: "editor@test.com".to_string(),
            name: "Editor".to_string(),
            picture: None,
        },
    );
    let state = Arc::new(AppState {
        oauth_client: backend::auth::build_oauth_client(),
        editor_emails: vec!["editor@test.com".to_string()],
        pool,
        sessions: Mutex::new(sessions),
        frontend_url: "http://localhost:5173".to_string(),
    });

    let app = routes::router().with_state(state);
    TestServer::new(app)
}

// --- Days ---

#[tokio::test]
async fn test_get_days_empty() {
    let server = setup_server(setup_pool().await).await;

    let resp = server.get("/api/days").await;
    resp.assert_status_ok();

    let body: Vec<serde_json::Value> = resp.json();
    assert!(body.is_empty());
}

#[tokio::test]
async fn test_create_and_get_day() {
    let server = setup_server(setup_pool().await).await;

    let resp = server
        .post("/api/days")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({
            "date": "2026-10-01",
            "city_key": "beijing",
            "accommodation_key": "hotel-a"
        }))
        .await;
    resp.assert_status(axum::http::StatusCode::CREATED);

    let day: serde_json::Value = resp.json();
    assert_eq!(day["date"], "2026-10-01");
    assert_eq!(day["city_key"], "beijing");
    assert_eq!(day["accommodation_key"], "hotel-a");

    let id = day["id"].as_i64().unwrap();

    let resp = server.get(&format!("/api/days/{id}")).await;
    resp.assert_status_ok();

    let fetched: serde_json::Value = resp.json();
    assert_eq!(fetched["id"], id);
}

#[tokio::test]
async fn test_update_day() {
    let server = setup_server(setup_pool().await).await;

    let resp = server
        .post("/api/days")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({
            "date": "2026-10-01",
            "city_key": "beijing",
            "accommodation_key": null
        }))
        .await;
    let day: serde_json::Value = resp.json();
    let id = day["id"].as_i64().unwrap();

    let resp = server
        .put(&format!("/api/days/{id}"))
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({
            "notes": "Arrived in Beijing!",
            "emoji": "🏯"
        }))
        .await;
    resp.assert_status_ok();

    let updated: serde_json::Value = resp.json();
    assert_eq!(updated["notes"], "Arrived in Beijing!");
    assert_eq!(updated["emoji"], "🏯");
    assert_eq!(updated["city_key"], "beijing");
}

#[tokio::test]
async fn test_delete_day() {
    let server = setup_server(setup_pool().await).await;

    let resp = server
        .post("/api/days")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({
            "date": "2026-10-01",
            "city_key": "beijing",
            "accommodation_key": null
        }))
        .await;
    let day: serde_json::Value = resp.json();
    let id = day["id"].as_i64().unwrap();

    server
        .delete(&format!("/api/days/{id}"))
        .add_cookie(editor_cookie())
        .await
        .assert_status_ok();

    let resp = server.get(&format!("/api/days/{id}")).await;
    resp.assert_status(axum::http::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_day_not_found() {
    let server = setup_server(setup_pool().await).await;

    let resp = server.get("/api/days/999").await;
    resp.assert_status(axum::http::StatusCode::NOT_FOUND);
}

// --- Cities ---

#[tokio::test]
async fn test_get_cities() {
    let server = setup_server(setup_pool().await).await;

    let resp = server.get("/api/cities").await;
    resp.assert_status_ok();

    let cities: Vec<serde_json::Value> = resp.json();
    assert_eq!(cities.len(), 2);
}

#[tokio::test]
async fn test_get_city() {
    let server = setup_server(setup_pool().await).await;

    let resp = server.get("/api/cities/beijing").await;
    resp.assert_status_ok();

    let city: serde_json::Value = resp.json();
    assert_eq!(city["name"], "Beijing");
    assert_eq!(city["chinese_name"], "北京");
}

#[tokio::test]
async fn test_get_city_not_found() {
    let server = setup_server(setup_pool().await).await;

    let resp = server.get("/api/cities/tokyo").await;
    resp.assert_status(axum::http::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_city() {
    let server = setup_server(setup_pool().await).await;

    let resp = server
        .put("/api/cities/beijing")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({
            "notes": "Capital city",
            "emoji": "🇨🇳"
        }))
        .await;
    resp.assert_status_ok();

    let city: serde_json::Value = resp.json();
    assert_eq!(city["notes"], "Capital city");
    assert_eq!(city["emoji"], "🇨🇳");
    assert_eq!(city["name"], "Beijing");
}

// --- Accommodations ---

#[tokio::test]
async fn test_get_accommodations() {
    let server = setup_server(setup_pool().await).await;

    let resp = server.get("/api/accommodations").await;
    resp.assert_status_ok();

    let accs: Vec<serde_json::Value> = resp.json();
    assert_eq!(accs.len(), 1);
    assert_eq!(accs[0]["name"], "Hotel Alpha");
}

#[tokio::test]
async fn test_create_accommodation() {
    let server = setup_server(setup_pool().await).await;

    let resp = server
        .post("/api/accommodations")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({
            "key": "hotel-b",
            "name": "Hotel Beta"
        }))
        .await;
    resp.assert_status(axum::http::StatusCode::CREATED);

    let acc: serde_json::Value = resp.json();
    assert_eq!(acc["key"], "hotel-b");
    assert_eq!(acc["name"], "Hotel Beta");
}

#[tokio::test]
async fn test_update_accommodation() {
    let server = setup_server(setup_pool().await).await;

    let resp = server
        .put("/api/accommodations/hotel-a")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({
            "link": "https://example.com",
            "notes": "Great place"
        }))
        .await;
    resp.assert_status_ok();

    let acc: serde_json::Value = resp.json();
    assert_eq!(acc["link"], "https://example.com");
    assert_eq!(acc["notes"], "Great place");
}

#[tokio::test]
async fn test_delete_accommodation() {
    let server = setup_server(setup_pool().await).await;

    server
        .delete("/api/accommodations/hotel-a")
        .add_cookie(editor_cookie())
        .await
        .assert_status_ok();

    let resp = server.get("/api/accommodations/hotel-a").await;
    resp.assert_status(axum::http::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_accommodation_not_found() {
    let server = setup_server(setup_pool().await).await;

    let resp = server
        .delete("/api/accommodations/nonexistent")
        .add_cookie(editor_cookie())
        .await;
    resp.assert_status(axum::http::StatusCode::NOT_FOUND);
}

// --- Export ---

#[tokio::test]
async fn test_export_requires_auth() {
    let server = setup_server(setup_pool().await).await;

    let resp = server.get("/api/export").await;
    resp.assert_status(axum::http::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_export_tsv_empty() {
    let server = setup_server(setup_pool().await).await;

    let resp = server.get("/api/export").add_cookie(editor_cookie()).await;
    resp.assert_status_ok();

    let text = resp.text();
    assert!(text.starts_with("date\tcity\taccommodation\n"));
    assert_eq!(text.lines().count(), 1);
}

#[tokio::test]
async fn test_export_tsv_with_data() {
    let server = setup_server(setup_pool().await).await;

    server
        .post("/api/days")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({
            "date": "2026-10-01",
            "city_key": "beijing",
            "accommodation_key": "hotel-a"
        }))
        .await;

    let resp = server.get("/api/export").add_cookie(editor_cookie()).await;
    let text = resp.text();

    assert_eq!(text.lines().count(), 2);
    assert!(text.contains("2026-10-01\tbeijing\thotel-a"));
}

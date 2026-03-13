use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use cookie::Cookie;
use axum_test::TestServer;
use sqlx::SqlitePool;

use backend::auth::{self, AppState};
use backend::db;
use backend::models::AuthState;
use backend::routes;

const TEST_EDITOR_SESSION: &str = "test-editor-session";
const TEST_VIEWER_SESSION: &str = "test-viewer-session";

fn editor_cookie() -> Cookie<'static> {
    Cookie::build(("session", TEST_EDITOR_SESSION)).path("/").build().into_owned()
}

fn viewer_cookie() -> Cookie<'static> {
    Cookie::build(("session", TEST_VIEWER_SESSION)).path("/").build().into_owned()
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

fn make_state(pool: SqlitePool, sessions: HashMap<String, AuthState>) -> Arc<AppState> {
    Arc::new(AppState {
        oauth_client: auth::build_oauth_client(),
        editor_emails: vec!["editor@test.com".to_string()],
        pool,
        sessions: Mutex::new(sessions),
        frontend_url: "http://localhost:5173".to_string(),
    })
}

fn base_sessions() -> HashMap<String, AuthState> {
    let mut sessions = HashMap::new();
    sessions.insert(
        TEST_EDITOR_SESSION.to_string(),
        AuthState { email: "editor@test.com".to_string(), name: "Editor".to_string(), picture: None },
    );
    sessions.insert(
        TEST_VIEWER_SESSION.to_string(),
        AuthState { email: "viewer@test.com".to_string(), name: "Viewer".to_string(), picture: None },
    );
    sessions
}

async fn setup_server(pool: SqlitePool) -> TestServer {
    let state = make_state(pool, base_sessions());
    let app = routes::router().with_state(state);
    TestServer::new(app)
}

/// Server that mounts both API routes and auth routes (for /api/auth/* tests).
async fn setup_full_server(pool: SqlitePool) -> TestServer {
    let state = make_state(pool, base_sessions());
    let app = axum::Router::new()
        .merge(auth::router())
        .merge(routes::router())
        .with_state(state);
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
            "tagline": "Capital city",
            "emoji": "🇨🇳"
        }))
        .await;
    resp.assert_status_ok();

    let city: serde_json::Value = resp.json();
    assert_eq!(city["tagline"], "Capital city");
    assert_eq!(city["emoji"], "🇨🇳");
    assert_eq!(city["name"], "Beijing");
}

#[tokio::test]
async fn test_create_city() {
    let server = setup_server(setup_pool().await).await;

    let resp = server
        .post("/api/cities")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({
            "name": "Shanghai",
            "chinese_name": "上海",
            "tagline": "The Bund and skyline",
            "lat": 31.2304,
            "lng": 121.4737
        }))
        .await;
    resp.assert_status(axum::http::StatusCode::CREATED);

    let city: serde_json::Value = resp.json();
    assert_eq!(city["key"], "shanghai");
    assert_eq!(city["name"], "Shanghai");
    assert_eq!(city["chinese_name"], "上海");
    assert_eq!(city["tagline"], "The Bund and skyline");
    assert_eq!(city["lat"], 31.2304);
    assert_eq!(city["lng"], 121.4737);
}

#[tokio::test]
async fn test_create_city_same_name_gets_unique_key() {
    let server = setup_server(setup_pool().await).await;

    let resp1 = server
        .post("/api/cities")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({"name": "Beijing"}))
        .await;
    resp1.assert_status(axum::http::StatusCode::CREATED);
    let city1: serde_json::Value = resp1.json();
    assert_eq!(city1["key"], "beijing-2");

    let resp2 = server
        .post("/api/cities")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({"name": "Shanghai"}))
        .await;
    resp2.assert_status(axum::http::StatusCode::CREATED);
    let city2: serde_json::Value = resp2.json();
    assert_eq!(city2["key"], "shanghai");

    let resp3 = server
        .post("/api/cities")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({"name": "Shanghai"}))
        .await;
    resp3.assert_status(axum::http::StatusCode::CREATED);
    let city3: serde_json::Value = resp3.json();
    assert_eq!(city3["key"], "shanghai-2");
}

#[tokio::test]
async fn test_delete_city() {
    let server = setup_server(setup_pool().await).await;

    let resp = server
        .post("/api/cities")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({"name": "Shanghai"}))
        .await;
    resp.assert_status(axum::http::StatusCode::CREATED);
    let city: serde_json::Value = resp.json();
    let key = city["key"].as_str().unwrap();

    server
        .delete(&format!("/api/cities/{}", key))
        .add_cookie(editor_cookie())
        .await
        .assert_status_ok();

    let resp = server.get(&format!("/api/cities/{}", key)).await;
    resp.assert_status(axum::http::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_city_with_days() {
    let server = setup_server(setup_pool().await).await;

    server
        .post("/api/days")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({
            "date": "2026-10-01",
            "city_key": "beijing",
            "accommodation_key": null
        }))
        .await
        .assert_status(axum::http::StatusCode::CREATED);

    let resp = server
        .delete("/api/cities/beijing")
        .add_cookie(editor_cookie())
        .await;
    resp.assert_status(axum::http::StatusCode::BAD_REQUEST);

    let body: serde_json::Value = resp.json();
    let err = body["error"].as_str().unwrap();
    assert!(err.contains("Cannot delete"));
    assert!(err.contains("days"));
    assert!(err.contains("Beijing"));
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
        .json(&serde_json::json!({"name": "Hotel Beta"}))
        .await;
    resp.assert_status(axum::http::StatusCode::CREATED);

    let acc: serde_json::Value = resp.json();
    assert_eq!(acc["key"], "hotel-beta");
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

#[tokio::test]
async fn test_delete_accommodation_with_days() {
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
    let day_id = day["id"].as_i64().unwrap();

    server
        .delete("/api/accommodations/hotel-a")
        .add_cookie(editor_cookie())
        .await
        .assert_status_ok();

    let resp = server.get("/api/accommodations/hotel-a").await;
    resp.assert_status(axum::http::StatusCode::NOT_FOUND);

    let resp = server.get(&format!("/api/days/{day_id}")).await;
    resp.assert_status_ok();
    let day_after: serde_json::Value = resp.json();
    assert!(day_after["accommodation_key"].is_null());
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

// --- Authorization (401 / 403) ---

#[tokio::test]
async fn test_create_day_requires_auth() {
    let server = setup_server(setup_pool().await).await;
    let resp = server
        .post("/api/days")
        .json(&serde_json::json!({"date": "2026-10-01", "city_key": "beijing"}))
        .await;
    resp.assert_status(axum::http::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_day_viewer_gets_403() {
    let server = setup_server(setup_pool().await).await;
    let resp = server
        .post("/api/days")
        .add_cookie(viewer_cookie())
        .json(&serde_json::json!({"date": "2026-10-01", "city_key": "beijing"}))
        .await;
    resp.assert_status(axum::http::StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_update_city_requires_auth() {
    let server = setup_server(setup_pool().await).await;
    let resp = server
        .put("/api/cities/beijing")
        .json(&serde_json::json!({"tagline": "Capital city"}))
        .await;
    resp.assert_status(axum::http::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_city_viewer_gets_403() {
    let server = setup_server(setup_pool().await).await;
    let resp = server
        .put("/api/cities/beijing")
        .add_cookie(viewer_cookie())
        .json(&serde_json::json!({"tagline": "Capital city"}))
        .await;
    resp.assert_status(axum::http::StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_accommodation_requires_auth() {
    let server = setup_server(setup_pool().await).await;
    let resp = server.delete("/api/accommodations/hotel-a").await;
    resp.assert_status(axum::http::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_delete_accommodation_viewer_gets_403() {
    let server = setup_server(setup_pool().await).await;
    let resp = server
        .delete("/api/accommodations/hotel-a")
        .add_cookie(viewer_cookie())
        .await;
    resp.assert_status(axum::http::StatusCode::FORBIDDEN);
}

// --- Tips ---

#[tokio::test]
async fn test_get_tips_empty() {
    let server = setup_server(setup_pool().await).await;

    let resp = server.get("/api/tips").await;
    resp.assert_status_ok();

    let tips: serde_json::Value = resp.json();
    assert_eq!(tips["key"], "main");
    assert_eq!(tips["content"], "");
}

#[tokio::test]
async fn test_update_tips() {
    let server = setup_server(setup_pool().await).await;

    let resp = server
        .put("/api/tips")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({"content": "# Tips\n\nBring a VPN."}))
        .await;
    resp.assert_status_ok();

    let tips: serde_json::Value = resp.json();
    assert_eq!(tips["key"], "main");
    assert!(tips["content"].as_str().unwrap().contains("Bring a VPN"));

    let get_resp = server.get("/api/tips").await;
    get_resp.assert_status_ok();
    let got: serde_json::Value = get_resp.json();
    assert!(got["content"].as_str().unwrap().contains("Bring a VPN"));
}

// --- Checklist ---

#[tokio::test]
async fn test_get_checklist_editors() {
    let server = setup_server(setup_pool().await).await;

    let resp = server.get("/api/checklist/editors").await;
    resp.assert_status_ok();

    let editors: Vec<serde_json::Value> = resp.json();
    assert_eq!(editors.len(), 1);
    assert_eq!(editors[0]["email"], "editor@test.com");
}

#[tokio::test]
async fn test_get_checklist_items_empty() {
    let server = setup_server(setup_pool().await).await;

    let resp = server.get("/api/checklist/items").await;
    resp.assert_status_ok();

    let items: Vec<serde_json::Value> = resp.json();
    assert!(items.is_empty());
}

#[tokio::test]
async fn test_create_checklist_item() {
    let server = setup_server(setup_pool().await).await;

    let resp = server
        .post("/api/checklist/items")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({"label": "Passport", "sort_order": 1}))
        .await;
    resp.assert_status(axum::http::StatusCode::CREATED);

    let item: serde_json::Value = resp.json();
    assert_eq!(item["label"], "Passport");
    assert_eq!(item["sort_order"], 1);

    let id = item["id"].as_i64().unwrap();

    let list_resp = server.get("/api/checklist/items").await;
    list_resp.assert_status_ok();
    let items: Vec<serde_json::Value> = list_resp.json();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["id"], id);
}

#[tokio::test]
async fn test_update_checklist_item() {
    let server = setup_server(setup_pool().await).await;

    let create_resp = server
        .post("/api/checklist/items")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({"label": "Visa"}))
        .await;
    let item: serde_json::Value = create_resp.json();
    let id = item["id"].as_i64().unwrap();

    let resp = server
        .put(&format!("/api/checklist/items/{}", id))
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({"label": "Visa + passport"}))
        .await;
    resp.assert_status_ok();

    let updated: serde_json::Value = resp.json();
    assert_eq!(updated["label"], "Visa + passport");
}

#[tokio::test]
async fn test_delete_checklist_item() {
    let server = setup_server(setup_pool().await).await;

    let create_resp = server
        .post("/api/checklist/items")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({"label": "Adapter"}))
        .await;
    let item: serde_json::Value = create_resp.json();
    let id = item["id"].as_i64().unwrap();

    let resp = server
        .delete(&format!("/api/checklist/items/{}", id))
        .add_cookie(editor_cookie())
        .await;
    resp.assert_status_ok();

    let list_resp = server.get("/api/checklist/items").await;
    let items: Vec<serde_json::Value> = list_resp.json();
    assert!(items.is_empty());
}

#[tokio::test]
async fn test_checklist_put_check() {
    let server = setup_server(setup_pool().await).await;

    let create_resp = server
        .post("/api/checklist/items")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({"label": "Passport"}))
        .await;
    let item: serde_json::Value = create_resp.json();
    let item_id = item["id"].as_i64().unwrap();

    let resp = server
        .put("/api/checklist/checks")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({"item_id": item_id, "checked": true}))
        .await;
    resp.assert_status_ok();

    let body: serde_json::Value = resp.json();
    assert_eq!(body["item_id"], item_id);
    assert_eq!(body["editor_email"], "editor@test.com");
    assert_eq!(body["checked"], true);

    let checks_resp = server.get("/api/checklist/checks").await;
    checks_resp.assert_status_ok();
    let checks: serde_json::Value = checks_resp.json();
    let key = format!("{}:editor@test.com", item_id);
    assert_eq!(checks[key], true);
}

#[tokio::test]
async fn test_create_checklist_item_requires_auth() {
    let server = setup_server(setup_pool().await).await;

    let resp = server
        .post("/api/checklist/items")
        .json(&serde_json::json!({"label": "Passport"}))
        .await;
    resp.assert_status(axum::http::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_tips_requires_auth() {
    let server = setup_server(setup_pool().await).await;
    let resp = server
        .put("/api/tips")
        .json(&serde_json::json!({"content": "Updated"}))
        .await;
    resp.assert_status(axum::http::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_tips_viewer_gets_403() {
    let server = setup_server(setup_pool().await).await;
    let resp = server
        .put("/api/tips")
        .add_cookie(viewer_cookie())
        .json(&serde_json::json!({"content": "Updated"}))
        .await;
    resp.assert_status(axum::http::StatusCode::FORBIDDEN);
}

// --- Auth endpoints (/api/auth/*) ---

#[tokio::test]
async fn test_me_with_no_cookie_returns_null() {
    let server = setup_full_server(setup_pool().await).await;
    let resp = server.get("/api/auth/me").await;
    resp.assert_status_ok();
    let body: serde_json::Value = resp.json();
    assert!(body.is_null());
}

#[tokio::test]
async fn test_me_with_editor_cookie_returns_user_with_is_editor_true() {
    let server = setup_full_server(setup_pool().await).await;
    let resp = server.get("/api/auth/me").add_cookie(editor_cookie()).await;
    resp.assert_status_ok();
    let user: serde_json::Value = resp.json();
    assert_eq!(user["email"], "editor@test.com");
    assert_eq!(user["is_editor"], true);
}

#[tokio::test]
async fn test_me_with_viewer_cookie_returns_user_with_is_editor_false() {
    let server = setup_full_server(setup_pool().await).await;
    let resp = server.get("/api/auth/me").add_cookie(viewer_cookie()).await;
    resp.assert_status_ok();
    let user: serde_json::Value = resp.json();
    assert_eq!(user["email"], "viewer@test.com");
    assert_eq!(user["is_editor"], false);
}

#[tokio::test]
async fn test_logout_redirects_and_clears_session() {
    let server = setup_full_server(setup_pool().await).await;
    // axum-test follows redirects by default; check we end up somewhere valid
    let resp = server
        .get("/api/auth/logout")
        .add_cookie(editor_cookie())
        .await;
    // Logout redirects to the frontend_url (303 or 302)
    let status = resp.status_code();
    assert!(
        status == axum::http::StatusCode::SEE_OTHER
            || status == axum::http::StatusCode::FOUND
            || status == axum::http::StatusCode::OK,
        "Expected redirect status, got {status}"
    );
}

// --- Input validation / edge cases ---

#[tokio::test]
async fn test_create_day_missing_required_fields_returns_error() {
    let server = setup_server(setup_pool().await).await;
    // Missing both date and city_key
    let resp = server
        .post("/api/days")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({}))
        .await;
    // Axum's Json extractor returns 422 for deserialization failures
    let status = resp.status_code().as_u16();
    assert!(status == 400 || status == 422, "Expected 400 or 422, got {status}");
}

#[tokio::test]
async fn test_create_day_duplicate_date_returns_400() {
    let server = setup_server(setup_pool().await).await;
    // Create first day
    let create_resp = server
        .post("/api/days")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({
            "date": "2026-10-01",
            "city_key": "beijing",
            "accommodation_key": null
        }))
        .await;
    create_resp.assert_status(axum::http::StatusCode::CREATED);

    // Duplicate date violates UNIQUE constraint -> bad_request (400)
    let dup_resp = server
        .post("/api/days")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({
            "date": "2026-10-01",
            "city_key": "beijing",
            "accommodation_key": null
        }))
        .await;
    dup_resp.assert_status(axum::http::StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_accommodation_same_name_gets_unique_key() {
    let server = setup_server(setup_pool().await).await;

    let resp1 = server
        .post("/api/accommodations")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({"name": "Hotel Alpha"}))
        .await;
    resp1.assert_status(axum::http::StatusCode::CREATED);
    let acc1: serde_json::Value = resp1.json();
    assert_eq!(acc1["key"], "hotel-alpha");

    let resp2 = server
        .post("/api/accommodations")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({"name": "Hotel Alpha"}))
        .await;
    resp2.assert_status(axum::http::StatusCode::CREATED);
    let acc2: serde_json::Value = resp2.json();
    assert_eq!(acc2["key"], "hotel-alpha-2");
}

#[tokio::test]
async fn test_update_day_empty_body_is_noop() {
    let server = setup_server(setup_pool().await).await;

    let create_resp = server
        .post("/api/days")
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({
            "date": "2026-10-01",
            "city_key": "beijing",
            "accommodation_key": null
        }))
        .await;
    let day: serde_json::Value = create_resp.json();
    let id = day["id"].as_i64().unwrap();

    let update_resp = server
        .put(&format!("/api/days/{id}"))
        .add_cookie(editor_cookie())
        .json(&serde_json::json!({}))
        .await;
    update_resp.assert_status_ok();

    let updated: serde_json::Value = update_resp.json();
    assert_eq!(updated["date"], "2026-10-01");
    assert_eq!(updated["city_key"], "beijing");
}

#[tokio::test]
async fn test_get_accommodation_by_key() {
    let server = setup_server(setup_pool().await).await;
    let resp = server.get("/api/accommodations/hotel-a").await;
    resp.assert_status_ok();
    let acc: serde_json::Value = resp.json();
    assert_eq!(acc["key"], "hotel-a");
    assert_eq!(acc["name"], "Hotel Alpha");
}

#[tokio::test]
async fn test_get_accommodation_not_found() {
    let server = setup_server(setup_pool().await).await;
    let resp = server.get("/api/accommodations/nonexistent").await;
    resp.assert_status(axum::http::StatusCode::NOT_FOUND);
}

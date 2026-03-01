use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::Router;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tower_http::services::ServeDir;

use backend::auth::{self, AppState};
use backend::db;
use backend::models::AuthState;
use backend::routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:trip.db?mode=rwc".to_string());

    let pool = db::init_pool(&database_url).await;
    db::run_migrations(&pool).await;

    if db::is_db_empty(&pool).await {
        let tsv_path =
            std::env::var("SEED_TSV_PATH").unwrap_or_else(|_| "../seed.tsv".to_string());
        db::seed_from_tsv(&pool, &tsv_path).await;
    }

    let editor_emails: Vec<String> = std::env::var("EDITOR_EMAILS")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let oauth_client = auth::build_oauth_client();

    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());

    // Sessions are in-memory: they are lost on restart and not shared across instances.
    // For production with multiple instances, use a shared store (e.g. Redis or DB).
    let mut initial_sessions = HashMap::new();
    if let Ok(test_session) = std::env::var("TEST_EDITOR_SESSION") {
        if let Some(email) = editor_emails.first().cloned() {
            initial_sessions.insert(test_session, AuthState {
                email,
                name: "Test Editor".to_string(),
                picture: None,
            });
        }
    }
    let app_state = Arc::new(AppState {
        oauth_client,
        editor_emails,
        pool: pool.clone(),
        sessions: Mutex::new(initial_sessions),
        frontend_url: frontend_url.clone(),
    });

    std::fs::create_dir_all("static").ok();

    let cors_origin = frontend_url
        .parse()
        .unwrap_or_else(|_| "http://localhost:5173".parse().unwrap());
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact(cors_origin))
        .allow_methods(AllowMethods::list([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
        ]))
        .allow_headers(AllowHeaders::list([
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
        ]))
        .allow_credentials(true)
        .max_age(std::time::Duration::from_secs(3600));

    let app = Router::new()
        .merge(auth::router())
        .merge(routes::router())
        .nest_service("/static", ServeDir::new("static"))
        .layer(cors)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("Starting server at http://localhost:8080");
    axum::serve(listener, app).await.unwrap();
}

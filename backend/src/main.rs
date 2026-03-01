use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::Router;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tower_http::services::ServeDir;

use backend::auth::{self, AppState};
use backend::db;
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

    let app_state = Arc::new(AppState {
        oauth_client,
        editor_emails,
        pool: pool.clone(),
        sessions: Mutex::new(HashMap::new()),
    });

    std::fs::create_dir_all("static").ok();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact(
            "http://localhost:5173".parse().unwrap(),
        ))
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

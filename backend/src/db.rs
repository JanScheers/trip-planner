use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

use crate::models::*;

pub async fn init_pool(database_url: &str) -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to SQLite")
}

pub async fn run_migrations(pool: &SqlitePool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS cities (
            key TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            chinese_name TEXT NOT NULL DEFAULT '',
            notes TEXT NOT NULL DEFAULT '',
            emoji TEXT,
            hero_image TEXT
        )",
    )
    .execute(pool)
    .await
    .expect("Failed to create cities table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS accommodations (
            key TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            link TEXT NOT NULL DEFAULT '',
            notes TEXT NOT NULL DEFAULT '',
            emoji TEXT,
            hero_image TEXT
        )",
    )
    .execute(pool)
    .await
    .expect("Failed to create accommodations table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS days (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL UNIQUE,
            city_key TEXT NOT NULL,
            accommodation_key TEXT,
            notes TEXT NOT NULL DEFAULT '',
            emoji TEXT,
            hero_image TEXT,
            FOREIGN KEY (city_key) REFERENCES cities(key),
            FOREIGN KEY (accommodation_key) REFERENCES accommodations(key)
        )",
    )
    .execute(pool)
    .await
    .expect("Failed to create days table");
}

pub async fn is_db_empty(pool: &SqlitePool) -> bool {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM days")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));
    count.0 == 0
}

pub async fn seed_from_tsv(pool: &SqlitePool, tsv_path: &str) {
    let content = std::fs::read_to_string(tsv_path).expect("Failed to read seed.tsv");
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(content.as_bytes());

    let city_names: std::collections::HashMap<&str, &str> = [
        ("beijing", "Beijing"),
        ("xian", "Xi'an"),
        ("chengdu", "Chengdu"),
        ("chongqing", "Chongqing"),
        ("zhangjiajie", "Zhangjiajie"),
        ("guilin", "Guilin"),
        ("hongkong", "Hong Kong"),
    ]
    .into_iter()
    .collect();

    for (key, name) in &city_names {
        sqlx::query(
            "INSERT OR IGNORE INTO cities (key, name, chinese_name, notes) VALUES (?, ?, '', '')",
        )
        .bind(key)
        .bind(name)
        .execute(pool)
        .await
        .expect("Failed to insert city");
    }

    for result in reader.records() {
        let record = result.expect("Failed to read TSV record");
        let date = &record[0];
        let city_key = &record[1];
        let accommodation_key = if record.len() > 2 && !record[2].is_empty() {
            Some(record[2].to_string())
        } else {
            None
        };

        sqlx::query(
            "INSERT OR IGNORE INTO days (date, city_key, accommodation_key, notes) VALUES (?, ?, ?, '')",
        )
        .bind(date)
        .bind(city_key)
        .bind(&accommodation_key)
        .execute(pool)
        .await
        .expect("Failed to insert day");
    }

    println!("Seeded database from {}", tsv_path);
}

pub async fn export_tsv(pool: &SqlitePool) -> String {
    let days: Vec<Day> = sqlx::query_as("SELECT * FROM days ORDER BY date")
        .fetch_all(pool)
        .await
        .expect("Failed to fetch days for export");

    let mut output = String::from("date\tcity\taccommodation\n");
    for day in days {
        output.push_str(&format!(
            "{}\t{}\t{}\n",
            day.date,
            day.city_key,
            day.accommodation_key.unwrap_or_default()
        ));
    }
    output
}

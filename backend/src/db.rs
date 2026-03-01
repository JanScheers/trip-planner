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
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Failed to run migrations");
}

pub async fn is_db_empty(pool: &SqlitePool) -> bool {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM days")
        .fetch_one(pool)
        .await
        .unwrap_or((0,));
    count.0 == 0
}

fn title_case_from_key(key: &str) -> String {
    key.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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

    // Collect all records so we can extract accommodation keys before inserting days.
    struct Record {
        date: String,
        city_key: String,
        accommodation_key: Option<String>,
    }

    let mut records: Vec<Record> = Vec::new();
    for result in reader.records() {
        let record = result.expect("Failed to read TSV record");
        let date = record[0].to_string();
        let city_key = record[1].to_string();
        let accommodation_key = if record.len() > 2 && !record[2].is_empty() {
            Some(record[2].to_string())
        } else {
            None
        };
        records.push(Record { date, city_key, accommodation_key });
    }

    // Seed unique accommodations derived from the TSV (key -> title-cased name).
    let mut seen_accommodation_keys = std::collections::HashSet::new();
    for rec in &records {
        if let Some(key) = &rec.accommodation_key {
            if seen_accommodation_keys.insert(key.clone()) {
                let name = title_case_from_key(key);
                sqlx::query(
                    "INSERT OR IGNORE INTO accommodations (key, name, link, notes) VALUES (?, ?, '', '')",
                )
                .bind(key)
                .bind(&name)
                .execute(pool)
                .await
                .expect("Failed to insert accommodation");
            }
        }
    }

    // Insert days.
    for rec in &records {
        sqlx::query(
            "INSERT OR IGNORE INTO days (date, city_key, accommodation_key, notes) VALUES (?, ?, ?, '')",
        )
        .bind(&rec.date)
        .bind(&rec.city_key)
        .bind(&rec.accommodation_key)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    async fn test_pool() -> SqlitePool {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        run_migrations(&pool).await;
        pool
    }

    #[tokio::test]
    async fn is_db_empty_true_on_fresh_db() {
        let pool = test_pool().await;
        assert!(is_db_empty(&pool).await);
    }

    #[tokio::test]
    async fn is_db_empty_false_after_inserting_day() {
        let pool = test_pool().await;
        sqlx::query("INSERT INTO cities (key, name, chinese_name) VALUES ('bj', 'Beijing', '北京')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO days (date, city_key, notes) VALUES ('2026-10-01', 'bj', '')")
            .execute(&pool)
            .await
            .unwrap();
        assert!(!is_db_empty(&pool).await);
    }

    #[tokio::test]
    async fn export_tsv_returns_header_only_when_empty() {
        let pool = test_pool().await;
        let output = export_tsv(&pool).await;
        assert_eq!(output, "date\tcity\taccommodation\n");
    }

    #[tokio::test]
    async fn export_tsv_includes_inserted_days() {
        let pool = test_pool().await;
        sqlx::query("INSERT INTO cities (key, name, chinese_name) VALUES ('bj', 'Beijing', '北京')")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query(
            "INSERT INTO days (date, city_key, accommodation_key, notes) VALUES ('2026-10-01', 'bj', NULL, '')",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO days (date, city_key, accommodation_key, notes) VALUES ('2026-10-02', 'bj', NULL, '')",
        )
        .execute(&pool)
        .await
        .unwrap();

        let output = export_tsv(&pool).await;
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "date\tcity\taccommodation");
        assert!(lines[1].starts_with("2026-10-01\tbj\t"));
        assert!(lines[2].starts_with("2026-10-02\tbj\t"));
    }

    #[tokio::test]
    async fn seed_from_tsv_inserts_cities_accommodations_and_days() {
        let pool = test_pool().await;

        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        writeln!(tmpfile, "date\tcity\taccommodation").unwrap();
        writeln!(tmpfile, "2026-10-01\tbeijing\thotel-alpha").unwrap();
        writeln!(tmpfile, "2026-10-02\tbeijing\thotel-alpha").unwrap();
        writeln!(tmpfile, "2026-10-03\txian\thotel-beta").unwrap();

        seed_from_tsv(&pool, tmpfile.path().to_str().unwrap()).await;

        let city_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM cities")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert!(city_count.0 >= 2, "Expected at least 2 cities");

        let acc_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM accommodations")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(acc_count.0, 2, "Expected hotel-alpha and hotel-beta");

        let day_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM days")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(day_count.0, 3);
    }

    #[tokio::test]
    async fn seed_from_tsv_generates_accommodation_names_from_key() {
        let pool = test_pool().await;

        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        writeln!(tmpfile, "date\tcity\taccommodation").unwrap();
        writeln!(tmpfile, "2026-10-01\tbeijing\tbeijing-hutong").unwrap();

        seed_from_tsv(&pool, tmpfile.path().to_str().unwrap()).await;

        let acc: (String,) = sqlx::query_as("SELECT name FROM accommodations WHERE key = 'beijing-hutong'")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(acc.0, "Beijing Hutong");
    }

    #[tokio::test]
    async fn seed_then_export_round_trip() {
        let pool = test_pool().await;

        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        writeln!(tmpfile, "date\tcity\taccommodation").unwrap();
        writeln!(tmpfile, "2026-10-01\tbeijing\thotel-alpha").unwrap();
        writeln!(tmpfile, "2026-10-02\tbeijing\t").unwrap();

        seed_from_tsv(&pool, tmpfile.path().to_str().unwrap()).await;

        let output = export_tsv(&pool).await;
        assert!(output.contains("2026-10-01\tbeijing\thotel-alpha"));
        assert!(output.contains("2026-10-02\tbeijing\t"));
    }

    #[test]
    fn title_case_from_key_splits_and_capitalizes() {
        assert_eq!(title_case_from_key("beijing-hutong"), "Beijing Hutong");
        assert_eq!(title_case_from_key("chengdu-panda-inn"), "Chengdu Panda Inn");
        assert_eq!(title_case_from_key("xian"), "Xian");
    }
}

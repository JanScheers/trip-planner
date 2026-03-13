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

pub async fn seed_from_dir(pool: &SqlitePool, seed_dir: &str) {
    seed_cities(pool, &format!("{}/cities.tsv", seed_dir)).await;
    seed_accommodations(pool, &format!("{}/accommodations.tsv", seed_dir)).await;
    seed_days(pool, &format!("{}/days.tsv", seed_dir)).await;
    seed_checklist_items(pool, &format!("{}/checklist_items.tsv", seed_dir)).await;
    seed_tips(pool, &format!("{}/tips.md", seed_dir)).await;
    println!("Seeded database from {}", seed_dir);
}

async fn seed_cities(pool: &SqlitePool, path: &str) {
    let content = std::fs::read_to_string(path).expect("Failed to read cities.tsv");
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(content.as_bytes());

    // columns: key, name, chinese_name, emoji, description, tagline, lat, lng, hero_image
    for result in reader.records() {
        let record = result.expect("Failed to read cities.tsv record");
        let key = record.get(0).unwrap_or("").to_string();
        let name = record.get(1).unwrap_or("").to_string();
        let chinese_name = record.get(2).unwrap_or("").to_string();
        let emoji: Option<String> = record.get(3).filter(|v| !v.is_empty()).map(str::to_string);
        let description = record.get(4).unwrap_or("").to_string();
        let tagline = record.get(5).unwrap_or("").to_string();
        let lat: Option<f64> = record.get(6).and_then(|v| v.parse().ok());
        let lng: Option<f64> = record.get(7).and_then(|v| v.parse().ok());
        let hero_image: Option<String> = record.get(8).filter(|v| !v.is_empty()).map(str::to_string);

        sqlx::query(
            "INSERT OR IGNORE INTO cities \
             (key, name, chinese_name, emoji, description, tagline, lat, lng, hero_image) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&key)
        .bind(&name)
        .bind(&chinese_name)
        .bind(&emoji)
        .bind(&description)
        .bind(&tagline)
        .bind(lat)
        .bind(lng)
        .bind(&hero_image)
        .execute(pool)
        .await
        .expect("Failed to insert city");
    }
}

async fn seed_accommodations(pool: &SqlitePool, path: &str) {
    let content = std::fs::read_to_string(path).expect("Failed to read accommodations.tsv");
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(content.as_bytes());

    // columns: key, name, emoji, notes, hero_image
    for result in reader.records() {
        let record = result.expect("Failed to read accommodations.tsv record");
        let key = record.get(0).unwrap_or("").to_string();
        let name = record.get(1).unwrap_or("").to_string();
        let emoji: Option<String> = record.get(2).filter(|v| !v.is_empty()).map(str::to_string);
        let notes = record.get(3).unwrap_or("").to_string();
        let hero_image: Option<String> = record.get(4).filter(|v| !v.is_empty()).map(str::to_string);

        sqlx::query(
            "INSERT OR IGNORE INTO accommodations (key, name, link, emoji, notes, hero_image) \
             VALUES (?, ?, '', ?, ?, ?)",
        )
        .bind(&key)
        .bind(&name)
        .bind(&emoji)
        .bind(&notes)
        .bind(&hero_image)
        .execute(pool)
        .await
        .expect("Failed to insert accommodation");
    }
}

async fn seed_days(pool: &SqlitePool, path: &str) {
    let content = std::fs::read_to_string(path).expect("Failed to read days.tsv");
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(content.as_bytes());

    // columns: date, city_key, accommodation_key, emoji, notes, tagline, travel, hero_image (optional)
    for result in reader.records() {
        let record = result.expect("Failed to read days.tsv record");
        let date = record.get(0).unwrap_or("").to_string();
        let city_key = record.get(1).unwrap_or("").to_string();
        let accommodation_key: Option<String> =
            record.get(2).filter(|v| !v.is_empty()).map(str::to_string);
        let emoji: Option<String> = record.get(3).filter(|v| !v.is_empty()).map(str::to_string);
        let notes = record.get(4).unwrap_or("").to_string();
        let tagline = record.get(5).unwrap_or("").to_string();
        let travel: Option<String> = record.get(6).filter(|v| !v.is_empty()).map(str::to_string);
        let hero_image: Option<String> = record.get(7).filter(|v| !v.is_empty()).map(str::to_string);

        sqlx::query(
            "INSERT OR IGNORE INTO days (date, city_key, accommodation_key, emoji, notes, tagline, travel, hero_image) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&date)
        .bind(&city_key)
        .bind(&accommodation_key)
        .bind(&emoji)
        .bind(&notes)
        .bind(&tagline)
        .bind(&travel)
        .bind(&hero_image)
        .execute(pool)
        .await
        .expect("Failed to insert day");
    }
}

async fn seed_tips(pool: &SqlitePool, path: &str) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };
    sqlx::query("INSERT OR IGNORE INTO tips (key, content) VALUES ('main', ?)")
        .bind(&content)
        .execute(pool)
        .await
        .expect("Failed to insert tips");
}

async fn seed_checklist_items(pool: &SqlitePool, path: &str) {
    let Ok(content) = std::fs::read_to_string(path) else {
        return;
    };
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(content.as_bytes());

    // columns: label, sort_order
    for result in reader.records() {
        let record = match result {
            Ok(r) => r,
            Err(_) => continue,
        };
        let label = record.get(0).unwrap_or("").to_string();
        let sort_order: i64 = record.get(1).and_then(|v| v.parse().ok()).unwrap_or(0);

        if label.is_empty() {
            continue;
        }

        sqlx::query("INSERT INTO checklist_items (label, sort_order) VALUES (?, ?)")
        .bind(&label)
        .bind(sort_order)
        .execute(pool)
        .await
        .expect("Failed to insert checklist item");
    }
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

    fn make_seed_dir() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    fn write_cities(dir: &tempfile::TempDir, rows: &[(&str, &str, &str)]) {
        let path = dir.path().join("cities.tsv");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "key\tname\tchinese_name\temoji\tdescription\ttagline\tlat\tlng\thero_image").unwrap();
        for (key, name, chinese_name) in rows {
            writeln!(f, "{}\t{}\t{}\t\tA great city.\tTagline.\t0\t0\t", key, name, chinese_name)
                .unwrap();
        }
    }

    fn write_accommodations(dir: &tempfile::TempDir, rows: &[(&str, &str)]) {
        let path = dir.path().join("accommodations.tsv");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "key\tname\temoji\tnotes\thero_image").unwrap();
        for (key, name) in rows {
            writeln!(f, "{}\t{}\t\t\t", key, name).unwrap();
        }
    }

    fn write_days(dir: &tempfile::TempDir, rows: &[(&str, &str, &str)]) {
        let path = dir.path().join("days.tsv");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "date\tcity_key\taccommodation_key\temoji\tnotes\ttagline\ttravel\thero_image").unwrap();
        for (date, city_key, acc_key) in rows {
            writeln!(f, "{}\t{}\t{}\t\t\t\t\t", date, city_key, acc_key).unwrap();
        }
    }

    fn write_tips(dir: &tempfile::TempDir, content: &str) {
        let path = dir.path().join("tips.md");
        std::fs::write(&path, content).unwrap();
    }

    fn write_checklist_items(dir: &tempfile::TempDir, rows: &[(&str, i64)]) {
        let path = dir.path().join("checklist_items.tsv");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "label\tsort_order").unwrap();
        for (label, sort_order) in rows {
            writeln!(f, "{}\t{}", label, sort_order).unwrap();
        }
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
    async fn seed_from_dir_inserts_cities_with_description_and_coords() {
        let pool = test_pool().await;
        let dir = make_seed_dir();

        let cities_path = dir.path().join("cities.tsv");
        let mut f = std::fs::File::create(&cities_path).unwrap();
        writeln!(f, "key\tname\tchinese_name\temoji\tdescription\ttagline\tlat\tlng\thero_image").unwrap();
        writeln!(f, "beijing\tBeijing\t北京\t🏯\tA grand imperial city.\tForbidden City.\t39.9\t116.4\t")
            .unwrap();
        writeln!(f, "xian\tXi'an\t西安\t🏺\tAncient Silk Road capital.\tTerracotta.\t34.3\t108.9\t")
            .unwrap();

        write_accommodations(&dir, &[]);
        write_days(&dir, &[]);

        seed_from_dir(&pool, dir.path().to_str().unwrap()).await;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM cities")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count.0, 2);

        let row: (String, String, Option<String>, String, String, Option<f64>, Option<f64>) =
            sqlx::query_as(
                "SELECT name, chinese_name, emoji, description, tagline, lat, lng \
                 FROM cities WHERE key = 'beijing'",
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row.0, "Beijing");
        assert_eq!(row.1, "北京");
        assert_eq!(row.2.as_deref(), Some("🏯"));
        assert_eq!(row.3, "A grand imperial city.");
        assert_eq!(row.4, "Forbidden City.");
        assert!((row.5.unwrap() - 39.9).abs() < 0.01);
        assert!((row.6.unwrap() - 116.4).abs() < 0.01);
    }

    #[tokio::test]
    async fn seed_from_dir_inserts_accommodations_and_days() {
        let pool = test_pool().await;
        let dir = make_seed_dir();

        write_cities(&dir, &[("beijing", "Beijing", "北京"), ("xian", "Xi'an", "西安")]);
        write_accommodations(&dir, &[("hotel-alpha", "Hotel Alpha"), ("hotel-beta", "Hotel Beta")]);
        write_days(&dir, &[
            ("2026-10-01", "beijing", "hotel-alpha"),
            ("2026-10-02", "beijing", "hotel-alpha"),
            ("2026-10-03", "xian", "hotel-beta"),
        ]);

        seed_from_dir(&pool, dir.path().to_str().unwrap()).await;

        let acc_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM accommodations")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(acc_count.0, 2);

        let day_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM days")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(day_count.0, 3);
    }

    #[tokio::test]
    async fn seed_from_dir_days_carry_emoji_and_notes() {
        let pool = test_pool().await;
        let dir = make_seed_dir();

        write_cities(&dir, &[("beijing", "Beijing", "北京")]);
        write_accommodations(&dir, &[("hutong", "Beijing Hutong")]);

        let days_path = dir.path().join("days.tsv");
        let mut f = std::fs::File::create(&days_path).unwrap();
        writeln!(f, "date\tcity_key\taccommodation_key\temoji\tnotes\ttagline\ttravel\thero_image").unwrap();
        writeln!(f, "2026-10-09\tbeijing\thutong\t✈️\tArrival day, explore hutongs.\tTouch down and wander the hutongs\t\t").unwrap();

        seed_from_dir(&pool, dir.path().to_str().unwrap()).await;

        let row: (Option<String>, String, String) =
            sqlx::query_as("SELECT emoji, notes, tagline FROM days WHERE date = '2026-10-09'")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(row.0.as_deref(), Some("✈️"));
        assert_eq!(row.1, "Arrival day, explore hutongs.");
        assert_eq!(row.2, "Touch down and wander the hutongs");
    }

    #[tokio::test]
    async fn seed_from_dir_inserts_checklist_items() {
        let pool = test_pool().await;
        let dir = make_seed_dir();

        write_cities(&dir, &[("beijing", "Beijing", "北京")]);
        write_accommodations(&dir, &[("hotel-a", "Hotel A")]);
        write_days(&dir, &[("2026-10-01", "beijing", "hotel-a")]);
        write_checklist_items(&dir, &[("Passport", 1), ("Visa", 2), ("Power adapter", 3)]);

        seed_from_dir(&pool, dir.path().to_str().unwrap()).await;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM checklist_items")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count.0, 3);

        let row: (String, i64) =
            sqlx::query_as("SELECT label, sort_order FROM checklist_items WHERE sort_order = 1")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(row.0, "Passport");
        assert_eq!(row.1, 1);
    }

    #[tokio::test]
    async fn seed_then_export_round_trip() {
        let pool = test_pool().await;
        let dir = make_seed_dir();

        write_cities(&dir, &[("beijing", "Beijing", "北京")]);
        write_accommodations(&dir, &[("hotel-alpha", "Hotel Alpha")]);
        write_days(&dir, &[
            ("2026-10-01", "beijing", "hotel-alpha"),
            ("2026-10-02", "beijing", ""),
        ]);

        seed_from_dir(&pool, dir.path().to_str().unwrap()).await;

        let output = export_tsv(&pool).await;
        assert!(output.contains("2026-10-01\tbeijing\thotel-alpha"));
        assert!(output.contains("2026-10-02\tbeijing\t"));
    }

    #[tokio::test]
    async fn seed_from_dir_inserts_tips() {
        let pool = test_pool().await;
        let dir = make_seed_dir();

        write_cities(&dir, &[("beijing", "Beijing", "北京")]);
        write_accommodations(&dir, &[("hotel-alpha", "Hotel Alpha")]);
        write_days(&dir, &[("2026-10-01", "beijing", "hotel-alpha")]);
        write_tips(&dir, "# China Tips\n\nBring a VPN.");

        seed_from_dir(&pool, dir.path().to_str().unwrap()).await;

        let row: (String, String) =
            sqlx::query_as("SELECT key, content FROM tips WHERE key = 'main'")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(row.0, "main");
        assert!(row.1.contains("China Tips"));
        assert!(row.1.contains("Bring a VPN"));
    }
}

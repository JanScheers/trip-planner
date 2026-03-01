CREATE TABLE IF NOT EXISTS cities (
    key TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    chinese_name TEXT NOT NULL DEFAULT '',
    notes TEXT NOT NULL DEFAULT '',
    emoji TEXT,
    hero_image TEXT
);

CREATE TABLE IF NOT EXISTS accommodations (
    key TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    link TEXT NOT NULL DEFAULT '',
    notes TEXT NOT NULL DEFAULT '',
    emoji TEXT,
    hero_image TEXT
);

CREATE TABLE IF NOT EXISTS days (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL UNIQUE,
    city_key TEXT NOT NULL,
    accommodation_key TEXT,
    notes TEXT NOT NULL DEFAULT '',
    emoji TEXT,
    hero_image TEXT,
    FOREIGN KEY (city_key) REFERENCES cities(key),
    FOREIGN KEY (accommodation_key) REFERENCES accommodations(key)
);

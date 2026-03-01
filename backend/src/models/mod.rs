use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, TS)]
#[ts(export)]
pub struct Day {
    pub id: i64,
    pub date: String,
    pub city_key: String,
    pub accommodation_key: Option<String>,
    pub notes: String,
    pub emoji: Option<String>,
    pub hero_image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CreateDay {
    pub date: String,
    pub city_key: String,
    pub accommodation_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UpdateDay {
    pub date: Option<String>,
    pub city_key: Option<String>,
    pub accommodation_key: Option<String>,
    pub notes: Option<String>,
    pub emoji: Option<String>,
    pub hero_image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, TS)]
#[ts(export)]
pub struct City {
    pub key: String,
    pub name: String,
    pub chinese_name: String,
    pub description: String,
    pub notes: String,
    pub emoji: Option<String>,
    pub hero_image: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UpdateCity {
    pub name: Option<String>,
    pub chinese_name: Option<String>,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub emoji: Option<String>,
    pub hero_image: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, TS)]
#[ts(export)]
pub struct Accommodation {
    pub key: String,
    pub name: String,
    pub link: String,
    pub notes: String,
    pub emoji: Option<String>,
    pub hero_image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CreateAccommodation {
    pub key: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UpdateAccommodation {
    pub name: Option<String>,
    pub link: Option<String>,
    pub notes: Option<String>,
    pub emoji: Option<String>,
    pub hero_image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UploadResponse {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AuthUser {
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
    pub is_editor: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthState {
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
}

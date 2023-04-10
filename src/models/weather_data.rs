// src/models/weather_data.rs

use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct WeatherData {
    pub station_id: i32,
    pub timestamp: NaiveDateTime,
    pub temperature: f32,
    pub humidity: f32,
    pub pm2_5_count: f32,
    pub pm10_count: f32,
    pub tvac: f32,
    pub aqi_index: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

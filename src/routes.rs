// src/routes.rs

use crate::models::weather_data::WeatherData;
use rocket::serde::json::Json;
use rocket::{get, post};

#[post("/v1/weather")]
pub async fn add_weather_data_v1() -> &'static str {
    "Received and stored weather data"
}

#[get("/v1/weather/<station_id>")]
pub async fn get_weather_data_v1(station_id: i32) -> Json<WeatherData> {
    Json(WeatherData {
        station_id,
        temperature: 20.5,
        humidity: 50.0,
        timestamp: chrono::Utc::now().naive_utc(),
        pm10_count: 12.0,
        pm2_5_count: 13.0,
        tvac: 11.0,
        aqi_index: 11.4,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    })
}

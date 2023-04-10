#![feature(proc_macro_hygiene, decl_macro)]

mod db;
mod models;
mod routes;
use rocket_dyn_templates::Template;
use routes::{add_weather_data_v1, get_weather_data_v1};

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let database_url = "postgres://aria_admin:aria_admin@localhost/weather_data_db";
    let pool = db::create_pool(database_url).await;

    rocket::build()
        .manage(pool)
        .mount("/api", routes![add_weather_data_v1, get_weather_data_v1])
        .launch()
        .await?;

    Ok(())
}

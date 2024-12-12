// src/lib.rs

#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use rocket::{Build, Rocket, State};
use sqlx::{MySql, Pool};
use std::env;

pub mod db_initializer;
pub mod db_utils;
pub mod users;

#[get("/")]
pub fn map_root() -> &'static str {
    "Map of Bartlesville recycling options"
}

#[get("/map")]
pub fn map() -> &'static str {
    "Map of Bartlesville recycling options"
}

#[get("/about")]
pub fn about() -> &'static str {
    "About Bville Recycle"
}

#[get("/db_test")]
pub async fn db_test(pool: &State<Pool<MySql>>) -> &'static str {
    match db_utils::verify_database_connection(pool.inner()).await {
        Ok(_) => "Database is working",
        Err(err) => {
            eprintln!("Database test failed: {}", err);
            "Database test failed"
        }
    }
}

pub async fn rocket() -> Rocket<Build> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Initialize the database if needed
    if let Err(err) = db_utils::initialize_database(&database_url).await {
        panic!("Failed to initialize database: {}", err);
    }

    let pool = Pool::<MySql>::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Verify database connection
    if let Err(err) = db_utils::verify_database_connection(&pool).await {
        panic!("Database connection verification failed: {}", err);
    }

    rocket::build()
        .manage(pool)
        .mount("/", routes![map_root, map, about, db_test])
}

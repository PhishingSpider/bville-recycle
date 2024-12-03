// src/lib.rs

#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket, State};
use sqlx::{MySql, Pool};
// use rocket::fairing::AdHoc;
use std::env;

// Route handlers
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

// Database interaction example route
#[get("/db_test")]
pub async fn db_test(pool: &State<Pool<MySql>>) -> &'static str {
    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(pool.inner())
        .await
        .expect("Failed to execute query");

    if row.0 == 1 {
        "Database is working"
    } else {
        "Database test failed"
    }
}

// Attach the database pool and configure routes
pub async fn rocket() -> Rocket<Build> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::<MySql>::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    rocket::build()
        .manage(pool)  // Add the pool to the Rocket state
        .mount("/", routes![map_root, map, about, db_test])
}

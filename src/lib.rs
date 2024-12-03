// src/lib.rs

#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket, State};
use sqlx::{MySql, Pool};


//Route handlers
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
    let pool = Pool::<MySql>::connect("mysql://runner:password@localhost/bville_recycle")
        .await
        .expect("Failed to connect to database");

    rocket::build()
        .manage(pool)  // Add the pool as a Rocket state
        .mount("/", routes![map_root, map, about, db_test])
}
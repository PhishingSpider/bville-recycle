// tests/crud.rs

// This file contains tests for basic CRUD operations on the database.
// These operations are for basic actions at a fundamental level, 
// NOT actions which are directly enactable by an inbound HTTP request.
// For actual actions enacted by users, these low-level operations are 
// to be enacted only after the user's request has been validated and
// authorized.


#![forbid(unsafe_code)]

use bville_recycle::{db_initializer, rocket};
use dotenvy::dotenv;
use rocket::local::asynchronous::Client as AsyncClient;
use sqlx::MySqlPool;
use std::env;

#[tokio::test]
async fn test_database_initialization() {
    dotenv().ok();
    
    let database_url: String = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let pool: sqlx::Pool<sqlx::MySql> = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    // Initialize the test database
    db_initializer::initialize_database(&pool, "sql/init.sql")
        .await
        .expect("Failed to initialize test database");

    // Your other test code
}



#[tokio::test]  // Tests the create_map_item function "/create_map_item"
async fn test_create_map_item() {
    let rocket: rocket::Rocket<rocket::Build> = rocket().await;
    let client: AsyncClient = AsyncClient::tracked(rocket).await.expect("valid rocket instance");
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // RESUME HERE
}
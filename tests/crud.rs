// tests/crud.rs

// This file contains tests for basic CRUD operations on the database.
// These operations are for basic actions at a fundamental level, 
// NOT actions which are directly enactable by an inbound HTTP request.

#![forbid(unsafe_code)]

use bville_recycle::rocket;
use rocket::local::asynchronous::Client as AsyncClient;
use sqlx::MySqlPool;

#[tokio::test]  // Tests the create_map_item function "/create_map_item"
async fn test_create_map_item() {
    let rocket = rocket().await;
    let client = AsyncClient::tracked(rocket).await.expect("valid rocket instance");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
}
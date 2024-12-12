// tests/crud.rs

// This file contains tests for basic CRUD operations on the database.
// These operations are for basic actions at a fundamental level, 
// NOT actions which are directly enactable by an inbound HTTP request.
// For actual actions enacted by users, these low-level operations are 
// to be enacted only after the user's request has been validated and
// authorized.


#![forbid(unsafe_code)]

use dotenvy::dotenv;
use std::env;
use bville_recycle::db_utils;

#[tokio::test]
async fn test_database_initialization() {
    dotenvy::from_filename(".env.test").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    db_utils::initialize_database(&database_url)
        .await
        .expect("Database initialization failed in test");
}


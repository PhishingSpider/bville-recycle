// tests/crud.rs

// This file contains tests for basic CRUD operations on the database.

#![forbid(unsafe_code)]

use bville_recycle::rocket;
use rocket::local::asynchronous::Client as AsyncClient;
use sqlx::MySqlPool;

#[tokio::test]  // Tests inserting a new recycling site
async fn test_insert_recycling_site() {
    let rocket = rocket().await;
    let client = AsyncClient::tracked(rocket).await.expect("valid rocket instance");

    let pool = client
        .rocket()
        .state::<MySqlPool>()
        .expect("Database pool not found");

    let result = sqlx::query("INSERT INTO recycling_sites (name, location) VALUES (?, ?)")
        .bind("Test Site")
        .bind("Test Location")
        .execute(pool)
        .await;

    assert!(result.is_ok(), "Failed to insert new recycling site");
}

#[tokio::test]  // Tests retrieving recycling sites
async fn test_get_recycling_sites() {
    let rocket = rocket().await;
    let client = AsyncClient::tracked(rocket).await.expect("valid rocket instance");

    let pool = client
        .rocket()
        .state::<MySqlPool>()
        .expect("Database pool not found");

    let sites: Vec<(String, String)> = sqlx::query_as("SELECT name, location FROM recycling_sites")
        .fetch_all(pool)
        .await
        .expect("Query execution failed");

    assert!(!sites.is_empty(), "No recycling sites found");
}

#[tokio::test]  // Tests updating a recycling site
async fn test_update_recycling_site() {
    let rocket = rocket().await;
    let client = AsyncClient::tracked(rocket).await.expect("valid rocket instance");

    let pool = client
        .rocket()
        .state::<MySqlPool>()
        .expect("Database pool not found");

    let result = sqlx::query("UPDATE recycling_sites SET location = ? WHERE name = ?")
        .bind("Updated Location")
        .bind("Test Site")
        .execute(pool)
        .await;

    assert!(result.is_ok(), "Failed to update recycling site");
}

#[tokio::test]  // Tests deleting a recycling site
async fn test_delete_recycling_site() {
    let rocket = rocket().await;
    let client = AsyncClient::tracked(rocket).await.expect("valid rocket instance");

    let pool = client
        .rocket()
        .state::<MySqlPool>()
        .expect("Database pool not found");

    let result = sqlx::query("DELETE FROM recycling_sites WHERE name = ?")
        .bind("Test Site")
        .execute(pool)
        .await;

    assert!(result.is_ok(), "Failed to delete recycling site");
}
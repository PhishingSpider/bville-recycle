// tests/routes.rs

use bville_recycle::rocket;
use rocket::http::Status; // Import HTTP status for response checks
use rocket::local::asynchronous::Client as AsyncClient;
// use rocket::local::blocking::Client as BlockClient; // Import the blocking client for testing // Import the rocket function from the library
use sqlx::MySqlPool;

#[tokio::test]  // Tests the map_root function "/"
async fn test_map_root() {
    let rocket = rocket().await;
    let client = AsyncClient::tracked(rocket).await.expect("valid rocket instance");
    let response = client.get("/").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().await,
        Some("Map of Bartlesville recycling options".into())
    );
}

#[tokio::test]  // Tests the map function "/map"
async fn test_map() {
    let rocket = rocket().await;
    let client = AsyncClient::tracked(rocket).await.expect("valid rocket instance");
    let response = client.get("/map").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().await,
        Some("Map of Bartlesville recycling options".into())
    );
}

#[tokio::test]  // Tests the about function "/about"
async fn test_about() {
    let rocket = rocket().await;
    let client = AsyncClient::tracked(rocket).await.expect("valid rocket instance");
    let response = client.get("/about").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().await,
        Some("About Bville Recycle".into())
    )
}

#[tokio::test]  // Use tokio for async tests
async fn test_db_connection() {
    let rocket = rocket().await;
    let client = AsyncClient::tracked(rocket).await.expect("valid rocket instance");
    
    let pool = client
        .rocket()
        .state::<MySqlPool>()
        .expect("Database pool not found");

    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(pool)
        .await
        .expect("Query execution failed");

    assert_eq!(row.0, 1, "Database connection test failed")
}


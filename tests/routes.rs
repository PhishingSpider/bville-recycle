// tests/routes.rs

use bville_recycle::rocket;
use dotenvy::dotenv;
use rocket::http::Status; // Import HTTP status for response checks
use rocket::local::asynchronous::Client as AsyncClient;
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

#[tokio::test]  // Tests the Database connection
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

#[tokio::test]  // Tests if the .env file loads
async fn test_load_env() {
    dotenv().expect("Failed to load .env file"); // Load environment variables from .env

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");

    println!("Loaded DATABASE_URL: {:?}", database_url);
}


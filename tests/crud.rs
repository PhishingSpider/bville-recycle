// tests/crud.rs

// This file contains tests for basic CRUD operations on the database.
// These operations are for basic actions at a fundamental level, 
// NOT actions which are directly enactable by an inbound HTTP request.
// For actual actions enacted by users, these low-level operations are 
// to be enacted only after the user's request has been validated and
// authorized.


#![forbid(unsafe_code)]

// use bville_recycle::{db_initializer, rocket};
use dotenvy::dotenv;
// use rocket::local::asynchronous::Client as AsyncClient;
// use sqlx::MySqlPool;
use std::env;
//use std::process::Command;
use url::Url;

#[tokio::test]
async fn test_database_initialization() {
    dotenv().ok();
    
    // Retrieve the DATABASE_URL from the environment
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Parse the DATABASE_URL using the `url` crate
    let pared_url: Url = Url::parse(&database_url).expect("Invalid DATABASE_URL format. Failed to parse DATABASE_URL");

    // Extract components from the parsed URL
    let scheme = pared_url.scheme();
    if scheme.is_empty() {
        panic!("Scheme not found in DATABASE_URL");
    }

    let username = pared_url.username();
    if username.is_empty() {
        panic!("Username not found in DATABASE_URL");
    }
    
    let password: &str = pared_url.password().expect("Password not found in DATABASE_URL"); 
    let host: &str = pared_url.host_str().expect("Host not found in DATABASE_URL");
    let path: &str = pared_url.path().trim_start_matches("/");
    if path.is_empty() {
        panic!("Path (database name) not found in DATABASE_URL");
    }

    // Print out the components
    println!("scheme: {}", scheme);
    println!("username: {}", username);
    println!("password: {}", password);
    println!("host: {}", host);
    println!("path: {}", path);

    // Configure test database
        "
        
        "


}




/* 
#[tokio::test]  // Tests the create_map_item function "/create_map_item"
async fn test_create_map_item() {
    let rocket: rocket::Rocket<rocket::Build> = rocket().await;
    let client: AsyncClient = AsyncClient::tracked(rocket).await.expect("valid rocket instance");
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        "
        sudo apt-get update
        sudo apt-get install -y mariadb-server
        sudo service mariadb start
        
        sudo mariadb -e \"CREATE DATABASE IF NOT EXISTS test_b_r;\"
        sudo mariadb -e \"DROP USER IF EXISTS '${{ secrets.DB_USER }}'@'localhost';\"
        sudo mariadb -e \"CREATE USER '${{ secrets.DB_USER }}'@'localhost';\"
        sudo mariadb -e \"SET PASSWORD FOR '${{ secrets.DB_USER }}'@'localhost' = PASSWORD('${{ secrets.DB_PASSWORD }}');\"
        sudo mariadb -e \"GRANT ALL PRIVILEGES ON bville_recycle.* TO '${{ secrets.DB_USER }}'@'localhost';\"
        sudo mariadb -e \"FLUSH PRIVILEGES;\"
        """

}
*/
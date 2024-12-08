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
use std::process::Command;
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

    // Configure test database
    let commands = [
        {
            println!("Updating package lists...");
            "sudo apt-get update -y"
        },
        {
            println!("Installing MariaDB server...");
            "sudo apt-get install -y mariadb-server"
        },
        {
            println!("Starting MariaDB service...");
            "sudo service mariadb start"
        },
        &format!("sudo mariadb -e \"CREATE DATABASE IF NOT EXISTS {path}\""),
        &format!("sudo mariadb -e \"DROP USER IF EXISTS '{username}'@'{host}'\""),
        &format!("sudo mariadb -e \"CREATE USER '{username}'@'{host}'\""),
        &format!("sudo mariadb -e \"SET PASSWORD FOR '{username}'@'{host}' = PASSWORD('{password}')\""),
        &format!("sudo mariadb -e \"GRANT ALL PRIVILEGES ON {path}.* TO '{username}'@'{host}'\""),
        &format!("sudo mariadb -e \"FLUSH PRIVILEGES\""),
    ];

    for cmd in commands {
        let output: std::process::Output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect(&format!("Failed to execute command: {}", cmd));

        if !output.status.success() {
            panic!("Command failed: {}\nError: {}", cmd, 
                String::from_utf8_lossy(&output.stderr));
        }
    }





}

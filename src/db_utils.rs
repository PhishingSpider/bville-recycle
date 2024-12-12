// src/db_utils.rs

#![forbid(unsafe_code)]

use dotenvy::dotenv;
use sqlx::MySqlPool;
use std::process::Command;
use url::Url;

pub async fn initialize_database(database_url: &str) -> Result<(), String> {
    dotenv().ok();

    let parsed_url = Url::parse(database_url).map_err(|e| format!("Invalid DATABASE_URL: {}", e))?;

    let username: &str = parsed_url.username();
    let password: &str = parsed_url.password().ok_or("Password not found in DATABASE_URL")?;
    let host: &str = parsed_url
        .host_str()
        .ok_or("Host not found in DATABASE_URL")?;
    let database_name = parsed_url.path().trim_start_matches('/');
    if database_name.is_empty() {
        return Err("Database name not found in DATABASE_URL".into());
    }

    // Commands to configure the database
    let commands = [
        &format!("sudo mariadb -e \"CREATE DATABASE IF NOT EXISTS {database_name}\""),
        &format!(
            "sudo mariadb -e \"DROP USER IF EXISTS '{username}'@'{host}'\""
        ),
        &format!(
            "sudo mariadb -e \"CREATE USER IF NOT EXISTS '{username}'@'{host}' IDENTIFIED BY '{password}'\""
        ),
        &format!(
            "sudo mariadb -e \"GRANT ALL PRIVILEGES ON {database_name}.* TO '{username}'@'{host}'\""
        ),
        "sudo mariadb -e \"FLUSH PRIVILEGES\"",
    ];

    for cmd in &commands {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|e| format!("Failed to execute command: {}. Error: {}", cmd, e))?;

        if !output.status.success() {
            return Err(format!(
                "Command failed: {}. Stderr: {}",
                cmd,
                String::from_utf8_lossy(&output.stderr)
            ));
        }
    }

    Ok(())
}


pub async fn verify_database_connection(pool: &MySqlPool) -> Result<(), String> {
    sqlx::query("SELECT 1")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Database connection test failed: {}", e))?;
    Ok(())
}

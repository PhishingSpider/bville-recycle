// src/db_initializer.rs

#![forbid(unsafe_code)]

use sqlx::{MySql, Pool};
use std::fs;

pub async fn initialize_database(pool: &Pool<MySql>, init_script: &str) -> Result<(), sqlx::Error> {
    // Check if the required table exists
    let table_check = sqlx::query("SHOW TABLES LIKE 'recycling_sites'")
        .fetch_optional(pool)
        .await?;

    // If the table doesn't exist, execute the initialization script
    if table_check.is_none() {
        println!("Recycling sites table not found. Initializing database...");
        let init_sql = fs::read_to_string(init_script)
            .expect("Failed to read database initialization script");
        sqlx::query(&init_sql).execute(pool).await?;
        println!("Database initialized successfully.");
    } else {
        println!("Database already initialized.");
    }

    Ok(())
}


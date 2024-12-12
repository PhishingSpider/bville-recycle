// src/users.rs

#![forbid(unsafe_code)]

use bville_recycle::rocket;
use regex::Regex;

pub struct Username {
    // Username may only contain uppercase letters, lowercase letters, numbers, and underscores
    pub username: String,
}

impl Username {
    pub fn new(username: String) -> Result<Username, &'static str> {
        if username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            if username.len() <= 32 {
                Ok(Username { username })
            } else {
                Err("Username must be 32 characters or less")
            }
        } else {
            Err("Username may only contain uppercase letters, lowercase letters, numbers, and underscores")
        }
    }
}

pub struct Email {
    // Email must be a valid email address
    pub email: String,
}

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        let email_regex = Regex::new(r"^[a-zA-Z\u0080-\uFFFF0-9._%+-]+@[a-zA-Z\u0080-\uFFFF0-9.-]+\.[a-zA-Z\u0080-\uFFFF]{1,}$").unwrap();

        if email_regex.is_match(email) {
            Ok(Self { 
                email: email.to_string(),
            })
        } else {
            Err(format!("'{}' is not a valid email address", email))
        }
    }
}

pub struct Password {
    // Password must be at least 16 characters long
    pub password: String,
}

impl Password {
    pub fn new(password: String) -> Result<Self, &'static str> {
        if password.len() >= 16 {
            if password.contains() {

            } else {
                Err("Password must contain at least one uppercase letter, one lowercase letter, one number, and one special character")
            }
            Ok(Self { 
                password: password.to_string(),
            })
        } else {
            Err("Password must be at least 16 characters long")
        }
    }
}

pub struct User {
    pub id: u32,
    pub username: Username,
    pub email: Email,
    pub password: Password,
    pub second_factor: Vec<u8>,
}



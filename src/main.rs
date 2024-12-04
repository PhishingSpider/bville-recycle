// src/main.rs

#![forbid(unsafe_code)]

use bville_recycle::rocket as app_rocket;
use rocket::launch;

#[launch]
async fn rocket() -> _ {
    app_rocket().await
}

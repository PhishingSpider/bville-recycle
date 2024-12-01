#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};

#[get("/favicon.ico")]
fn favicon() -> Option<rocket::fs::NamedFile> {
    rocket::fs::NamedFile::open("static/favicon.ico").ok()
}


#[get("/")]
fn index() -> &'static str {
    "Hello, Bartlesville!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/static", FileServer::from(relative!("static")))
}
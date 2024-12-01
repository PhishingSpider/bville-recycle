#![forbid(unsafe_code)]

#[macro_use] extern crate rocket;



#[get("/")]
fn map_root() -> &'static str {
    "Map of Bartlesville recycling options"
}

#[get("/map")]
fn map() -> &'static str {
    "Map of Bartlesville recycling options"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![map_root, map])
}

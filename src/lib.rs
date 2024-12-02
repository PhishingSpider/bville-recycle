#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

#[get("/")]
pub fn map_root() -> &'static str {
    "Map of Bartlesville recycling options"
}

#[get("/map")]
pub fn map() -> &'static str {
    "Map of Bartlesville recycling options"
}

pub fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![map_root, map])
}

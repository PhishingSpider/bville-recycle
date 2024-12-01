#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Bartlesville!"
}

#[get("/map")]
fn map() _> &'static str {
    "Map of Bartlesville recycling options"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
}
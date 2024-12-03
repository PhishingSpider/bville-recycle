// tests/routes.rs

use bville_recycle::rocket;
use rocket::http::Status; // Import HTTP status for response checks
use rocket::local::blocking::Client; // Import the blocking client for testing // Import the rocket function from the library

#[test]  // Tests the map_root function "/"
fn test_map_root() {
    let rocket = rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string(),
        Some("Map of Bartlesville recycling options".into())
    );
}

#[test]  // Tests the map function "/map"
fn test_map() {
    let rocket = rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let response = client.get("/map").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string(),
        Some("Map of Bartlesville recycling options".into())
    );
}

#[test]  // Tests the about function "/about"
fn test_about() {
    let rocket = rocket();
    let client = Client::tracked(rocket).expect("valid rocket instance");
    let response = client.get("/about").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string(),
        Some("About Bville Recycle".into())
    )
}

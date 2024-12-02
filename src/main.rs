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

/*
 _____         _   _               _____                
|_   _|__  ___| |_(_)_ __   __ _  |__  /___  _ __   ___ 
  | |/ _ \/ __| __| | '_ \ / _` |   / // _ \| '_ \ / _ \
  | |  __/\__ \ |_| | | | | (_| |  / /| (_) | | | |  __/
  |_|\___||___/\__|_|_| |_|\__, | /____\___/|_| |_|\___|
                           |___/                        
↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
*/

#[cfg(test)] // Ensure this block is only included during testing
mod tests {
    use super::*; // Import functions and routes from the main module
    use rocket::local::blocking::Client; // Import the blocking client for testing
    use rocket::http::Status; // Import HTTP status for response checks

    #[test]  // Tests the map_root function
    fn test_map_root() {
        let rocket = rocket::build()
            .mount("/", routes![map_root]);
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string(),
            Some("Map of Bartlesville recycling options".into())
        );
    }

    #[test]  // Tests the map function
    fn test_map() {
        let rocket = rocket::build()
            .mount("/", routes![map]);
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/map").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string(),
            Some("Map of Bartlesville recycling options".into())
        );
    }
}

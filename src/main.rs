#![forbid(unsafe_code)]

use bville_recycle::rocket as app_rocket;
use rocket::launch; // Rename the imported function

#[launch]
fn rocket() -> _ {
    app_rocket()
}

#[macro_use] extern crate rocket;
use ovenmitts::routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

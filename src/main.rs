#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket_db_pools::Database;

use ovenmitts::db::{run_migrations, Mitts};
use ovenmitts::routes::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Mitts::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
}

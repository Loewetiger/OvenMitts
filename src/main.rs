#[macro_use]
extern crate rocket;

use ovenmitts::auth::extract_permissions;
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;
use rocket_grants::GrantsFairing;

use ovenmitts::db::{run_migrations, Mitts};
use ovenmitts::routes::{get_user, post_admission, post_login, post_logout};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![post_admission, get_user, post_login, post_logout],
        )
        .attach(GrantsFairing::with_extractor_fn(|req| {
            Box::pin(extract_permissions(req))
        }))
        .attach(Mitts::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
}

#[macro_use]
extern crate rocket;

//use ovenmitts::auth::extract_permissions;
use rocket::fairing::AdHoc;
use rocket::figment::providers::{Env, Format, Toml};
use rocket::figment::value::{Map, Value};
use rocket::figment::{map, Figment, Profile};
use rocket_db_pools::Database;
//use rocket_grants::GrantsFairing;

use ovenmitts::db::{run_migrations, Mitts};
use ovenmitts::routes::{get_user, post_admission, post_login, post_logout, post_register};

#[launch]
fn rocket() -> _ {
    let db: Map<_, Value> = map! {
        "url" => "mitts.sqlite".into(),
    };
    let figment = Figment::from(rocket::Config::default())
        .merge(("databases", map!["mitts" => db]))
        .merge(Toml::file("Mitts.toml").nested())
        .merge(Env::prefixed("MITTS_").global())
        .select(Profile::from_env_or("APP_PROFILE", "default"));

    rocket::custom(figment)
        .mount(
            "/",
            routes![
                post_admission,
                get_user,
                post_login,
                post_logout,
                post_register
            ],
        )
        /*.attach(GrantsFairing::with_extractor_fn(|req| {
            Box::pin(extract_permissions(req))
        }))*/
        .attach(Mitts::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
}

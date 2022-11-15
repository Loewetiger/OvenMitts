#[macro_use]
extern crate rocket;

use ovenmitts::objects::Config;
use rocket::fairing::AdHoc;
use rocket::figment::providers::{Env, Format, Toml};
use rocket::figment::value::{Map, Value};
use rocket::figment::{map, Figment, Profile};
use rocket_db_pools::Database;

use ovenmitts::db::{run_migrations, Mitts};
use ovenmitts::routes::{
    get_streams, get_user, post_admission, post_login, post_logout, post_register,
};
use ovenmitts::static_files::{get_assets, get_index};

#[launch]
fn rocket() -> _ {
    let db: Map<_, Value> = map! {
        "url" => "mitts.sqlite".into(),
    };
    let figment = Figment::from(rocket::Config::default())
        .merge(("databases", map!["mitts" => db]))
        .merge(Toml::file(Env::var_or("MITTS_CONFIG", "mitts.toml")).profile("default"))
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
                post_register,
                get_streams,
                get_index,
                get_assets
            ],
        )
        .attach(Mitts::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
        .attach(AdHoc::config::<Config>())
}

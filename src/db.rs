use rocket::{fairing, Build, Rocket};
use rocket_db_pools::{sqlx, Database};

/// The main SQLite database.
#[derive(Database)]
#[database("mitts")]
pub struct Mitts(sqlx::SqlitePool);

/// Run the database migrations to make sure the right tables exist at any point.
pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Mitts::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

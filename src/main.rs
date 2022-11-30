use axum::{
    routing::{get, post},
    Router,
};
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tower_cookies::CookieManagerLayer;

use ovenmitts::{
    objects::{AppState, OMConfig},
    routes::{admission, list_users, login, logout, register, streams, update_user, user},
    static_files::{index, index_js, static_handler},
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let path = std::env::var("MITTS_CONFIG").unwrap_or("mitts.toml".into());
    let settings: OMConfig = Figment::new()
        .merge(Toml::file(path))
        .merge(Env::prefixed("MITTS_"))
        .extract()?;

    let options = SqliteConnectOptions::new()
        .filename(settings.database.clone())
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);
    let pool = SqlitePoolOptions::new().connect_with(options).await?;

    sqlx::migrate!().run(&pool).await?;

    let app = Router::new()
        .route("/admission", post(admission))
        .route("/user", get(user))
        .route("/user/login", post(login))
        .route("/user/logout", post(logout))
        .route("/user/register", post(register))
        .route("/user/list", get(list_users))
        .route("/user/update", post(update_user))
        .route("/streams", get(streams))
        .route("/", get(index))
        .route("/index.js", get(index_js))
        .route("/assets/*path", get(static_handler))
        .with_state(AppState {
            db: pool,
            config: settings.clone(),
        })
        .layer(CookieManagerLayer::new());

    axum::Server::bind(&settings.address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

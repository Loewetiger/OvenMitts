//! The routes for the webserver.

use chrono::DateTime;
use rocket::{
    http::{Cookie, CookieJar, SameSite, Status},
    response::status,
    serde::json::Json,
    time::{Duration, OffsetDateTime},
    State,
};
use rocket_db_pools::Connection;

use crate::{
    admission::handle_admission,
    crypto::{gen_stream_key, hash_password, verify_password},
    db::Mitts,
    objects::{
        Admission, AdmissionResponse, Config, LoginUser, ReqwestError, SendableUser, SessionCookie,
        StreamResp, Streams, User,
    },
    USERNAME_RE,
};

/// Used by OvenMediaEngine's [admission webhooks](https://airensoft.gitbook.io/ovenmediaengine/access-control/admission-webhooks).
///
/// Checks if the given stream key is in the database and rewrites the stream url to point to the users' username.
/// It does **not** currently check the headers for the HMAC signature provided by OME.
#[post("/admission", data = "<adm>")]
pub async fn post_admission(
    adm: Json<Admission>,
    db: Connection<Mitts>,
) -> Json<AdmissionResponse> {
    Json(handle_admission(adm.into_inner(), db).await)
}

/// Get information about the currently logged in user.
#[get("/user")]
#[must_use]
pub fn get_user(user: User) -> Json<SendableUser> {
    Json(user.into())
}

/// Login endpoint. Generates a session cookie and adds the token to the database.
#[post("/user/login", data = "<creds>")]
pub async fn post_login(
    creds: Json<LoginUser>,
    mut db: Connection<Mitts>,
    cookies: &CookieJar<'_>,
) -> Status {
    let user = match User::from_name(&creds.username, &mut *db).await {
        Some(u) => u,
        None => return Status::NotFound,
    };
    let valid_password =
        verify_password(&user.password, creds.password.as_bytes()).unwrap_or(false);
    if valid_password {
        let time = OffsetDateTime::now_utc() + Duration::days(14);
        let iso8601 = time
            .format(&rocket::time::format_description::well_known::iso8601::Iso8601::DEFAULT)
            .unwrap_or("1997-11-21T09:55:06.000000000-06:00".into());
        let cookie = SessionCookie::new(
            user.username,
            DateTime::parse_from_rfc3339(&iso8601).unwrap_or_default(),
        );

        let auth_cookie = Cookie::build("user_session", cookie)
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .expires(time);
        cookies.add_private(auth_cookie.finish());
        Status::Ok
    } else {
        Status::Unauthorized
    }
}

/// Logout endpoint. Deletes the session cookie, as well as the session from the database.
#[post("/user/logout")]
pub async fn post_logout(cookies: &CookieJar<'_>) -> Status {
    cookies.remove_private(Cookie::named("user_session"));
    Status::Ok
}

/// Register endpoint. Creates a new user in the database.
#[post("/user/register", data = "<creds>")]
pub async fn post_register(
    creds: Json<LoginUser>,
    mut db: Connection<Mitts>,
) -> Result<(), status::Custom<&'static str>> {
    if User::username_exists(&creds.username, &mut *db).await {
        return Err(status::Custom(Status::Conflict, "Username already exists"));
    }
    if !USERNAME_RE.is_match(&creds.username) {
        return Err(status::Custom(
            Status::BadRequest,
            "Username contains invalid characters",
        ));
    }

    let password_hash = match hash_password(creds.password.as_bytes()) {
        Ok(h) => h,
        Err(_) => {
            return Err(status::Custom(
                Status::InternalServerError,
                "Failed to hash password",
            ))
        }
    };
    let stream_key = gen_stream_key();
    if sqlx::query!(
        "INSERT INTO users(username, display_name, password, stream_key) VALUES(?, ?, ?, ?)",
        creds.username,
        creds.username,
        password_hash,
        stream_key
    )
    .execute(&mut *db)
    .await
    .is_err()
    {
        return Err(status::Custom(
            Status::InternalServerError,
            "Failed to create user",
        ));
    };
    Ok(())
}

/// Returns all currently active streams.
#[get("/streams")]
pub async fn get_streams(
    config: &State<Config>,
    mut db: Connection<Mitts>,
) -> Result<Json<Vec<StreamResp>>, ReqwestError> {
    let mut url = config.ome_url.clone();
    url.set_path("v1/vhosts/default/apps/stream/streams");

    let client = reqwest::Client::new();
    let body: Streams = client
        .get(url.as_str())
        .header(
            "authorization",
            format!("Basic {}", base64::encode(&config.access_token)),
        )
        .send()
        .await?
        .json()
        .await?;

    // Return early if there are no streams
    if body.response.is_empty() {
        return Ok(Json(Vec::new()));
    }

    let mut streams: Vec<StreamResp> = Vec::new();
    // SQLx sadly doesn't support IN queries, so we have to do this the hard way
    for s in body.response {
        match User::from_name(&s, &mut *db).await {
            Some(u) => streams.push(StreamResp {
                username: u.username,
                display_name: u.display_name,
                title: u.stream_title,
            }),
            None => (),
        };
    }
    Ok(Json(streams))
}

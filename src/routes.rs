//! The routes for the webserver.

use std::str::FromStr;

use chrono::DateTime;
use rocket::{
    http::{Cookie, CookieJar, SameSite, Status},
    serde::json::Json,
    time::{Duration, OffsetDateTime},
    State,
};
use rocket_db_pools::Connection;

use crate::{
    admission::handle_admission,
    crypto::{gen_stream_key, hash_password, verify_password},
    db::Mitts,
    errors::OMError,
    objects::{
        Admission, AdmissionResponse, Config, LoginUser, SendableUser, SessionCookie, StreamResp,
        Streams, User, UserUpdate,
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
) -> Result<(), OMError> {
    let user = match User::from_name(&creds.username, &mut *db).await {
        Some(u) => u,
        None => return Err(OMError::NotFound(creds.username.clone())),
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
        Ok(())
    } else {
        Err(OMError::InvalidSession)
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
) -> Result<(), OMError> {
    if User::username_exists(&creds.username, &mut *db).await {
        return Err(OMError::NameTaken);
    }
    if !USERNAME_RE.is_match(&creds.username) {
        return Err(OMError::InvalidName);
    }

    let password_hash =
        hash_password(creds.password.as_bytes()).map_err(|_| OMError::ArgonError)?;

    let stream_key = gen_stream_key();
    sqlx::query!(
        "INSERT INTO users(username, display_name, password, stream_key) VALUES(?, ?, ?, ?)",
        creds.username,
        creds.username,
        password_hash,
        stream_key
    )
    .execute(&mut *db)
    .await?;

    Ok(())
}

/// Change the settings of a user.
#[post("/user/update", data = "<body>")]
pub async fn update_user(
    mut db: Connection<Mitts>,
    cookies: &CookieJar<'_>,
    body: Json<UserUpdate>,
) -> Result<Status, OMError> {
    // warning: this is about to get ugly D:
    // get the current user
    let session_cookie = match cookies.get_private("user_session") {
        Some(c) => SessionCookie::from_str(c.value()).unwrap_or_default(),
        None => return Ok(Status::Unauthorized),
    };
    let logged_user = match session_cookie.get_user(&mut *db).await {
        Some(u) => u,
        None => return Ok(Status::Unauthorized),
    };

    // get the user to be updated
    if body.username.is_some() && !logged_user.is_admin() {
        return Ok(Status::Forbidden);
    }

    let user = match &body.username {
        Some(u) => User::from_name(u, &mut *db).await,
        None => User::from_name(&logged_user.username, &mut *db).await,
    };
    let user = match user {
        Some(u) => u,
        None => return Ok(Status::NotFound),
    };

    // check the individual fields
    if let Some(dn) = &body.display_name {
        sqlx::query!(
            "UPDATE users SET display_name = ? WHERE username = ?",
            dn,
            user.username
        )
        .execute(&mut *db)
        .await?;
    }

    if let Some(stream_title) = &body.stream_title {
        sqlx::query!(
            "UPDATE users SET stream_title = ? WHERE username = ?",
            stream_title,
            user.username
        )
        .execute(&mut *db)
        .await?;
    }

    let new_password: Option<String> = match (body.old_password.clone(), body.new_password.clone())
    {
        (None, Some(np)) => {
            if logged_user.is_admin() {
                hash_password(np.as_bytes()).ok()
            } else {
                return Err(OMError::NoPermission);
            }
        }
        (Some(op), Some(np)) => {
            if verify_password(&user.password, op.as_bytes()).unwrap_or(false) {
                hash_password(np.as_bytes()).ok()
            } else {
                return Err(OMError::InvalidPassword);
            }
        }
        _ => None,
    };
    if let Some(pw) = new_password {
        sqlx::query!(
            "UPDATE users SET password = ? WHERE username = ?",
            pw,
            user.username
        )
        .execute(&mut *db)
        .await?;
    };

    if logged_user.is_admin() {
        if let Some(permissions) = &body.permissions {
            sqlx::query!(
                "UPDATE users SET permissions = ? WHERE username = ?",
                permissions,
                user.username
            )
            .execute(&mut *db)
            .await?;
        }
    } else if body.permissions.is_some() {
        return Ok(Status::Forbidden);
    }
    Ok(Status::Ok)
}

/// Returns all the users in the database.
#[get("/user/list")]
pub async fn list_users(
    mut db: Connection<Mitts>,
    user: User,
) -> Result<Json<Vec<SendableUser>>, OMError> {
    if !user.is_admin() {
        return Err(OMError::NoPermission);
    }
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&mut *db)
        .await?;

    Ok(Json(
        users
            .into_iter()
            .map(|mut u| {
                u.stream_key = String::new();
                u.into()
            })
            .collect(),
    ))
}

/// Returns all currently active streams.
#[get("/streams")]
pub async fn get_streams(
    config: &State<Config>,
    mut db: Connection<Mitts>,
) -> Result<Json<Vec<StreamResp>>, OMError> {
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

//! The routes for the webserver.

use rocket::{
    http::{Cookie, CookieJar, SameSite, Status},
    response::status,
    serde::json::Json,
    time::{Duration, OffsetDateTime},
};
use rocket_db_pools::Connection;

use crate::{
    admission::handle_admission,
    crypto::{gen_stream_key, hash_password, random_data, verify_password},
    db::Mitts,
    objects::{Admission, AdmissionResponse, LoginUser, SendableUser, User},
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
    if cookies.get("user_session").is_some() {
        return Status::Ok;
    };
    let user = match User::from_name(&creds.username, &mut *db).await {
        Some(u) => u,
        None => return Status::NotFound,
    };
    let valid_password =
        verify_password(&user.password, creds.password.as_bytes()).unwrap_or(false);
    if valid_password {
        let token = base64::encode(random_data(64));
        if sqlx::query!(
            "INSERT INTO sessions(session, user_id) VALUES(?, ?)",
            token,
            user.id
        )
        .execute(&mut *db)
        .await
        .is_err()
        {
            return Status::InternalServerError;
        };

        let auth_cookie = Cookie::build("user_session", token)
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Lax)
            .expires(OffsetDateTime::now_utc() + Duration::days(14));
        cookies.add(auth_cookie.finish());
        Status::Ok
    } else {
        Status::Unauthorized
    }
}

/// Logout endpoint. Deletes the session cookie, as well as the session from the database.
#[post("/user/logout")]
pub async fn post_logout(cookies: &CookieJar<'_>, mut db: Connection<Mitts>) -> Status {
    match cookies.get("user_session") {
        Some(c) => {
            let token = c.value();
            cookies.remove(Cookie::named("user_session"));
            match sqlx::query!("DELETE FROM sessions WHERE session = ?", token)
                .execute(&mut *db)
                .await
            {
                Ok(_) => Status::Ok,
                Err(_) => Status::InternalServerError,
            }
        }
        None => Status::Ok,
    }
}

/// Register endpoint. Creates a new user in the database.
#[post("/user/register", data = "<creds>")]
pub async fn post_register(
    creds: Json<LoginUser>,
    mut db: Connection<Mitts>,
) -> Result<(), status::Custom<&'static str>> {
    let password_hash = match hash_password(creds.password.as_bytes()) {
        Ok(h) => h,
        Err(_) => {
            return Err(status::Custom(
                Status::InternalServerError,
                "Failed to hash password",
            ))
        }
    };
    let id = uuid::Uuid::new_v4().to_string();
    let stream_key = gen_stream_key();
    if sqlx::query!(
        "INSERT INTO users(id, username, password, stream_key) VALUES(?, ?, ?, ?)",
        id,
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

//! All the routes for the API.

use axum::{extract::State, Json};
use cookie::{time, SameSite};
use tokio::task::spawn_blocking;
use tower_cookies::{Cookie, Cookies};

use crate::{
    crypto::{gen_stream_key, hash_password, verify_password},
    errors::OMError,
    objects::{
        Admission, AdmissionResponse, OMConfig, SendableUser, StreamResp, Streams, User, UserLogin,
        UserUpdate,
    },
    Db, USERNAME_RE,
};

/// Handle the admission requests from the OvenMediaEngine server.
pub async fn admission(
    State(db): State<Db>,
    Json(adm): Json<Admission>,
) -> Json<AdmissionResponse> {
    let mut url = adm.borrow_url().clone();
    let mut path: Vec<&str> = match url.path_segments().map(std::iter::Iterator::collect) {
        Some(vec) => vec,
        None => return Json(AdmissionResponse::deny()),
    };
    // Get the last element, which should be the stream key
    let stream_key = path.pop().unwrap_or_default();
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE stream_key = ?", stream_key)
        .fetch_one(&db)
        .await;

    match user {
        Ok(user) => {
            if !user.has_permission("CAN_STREAM") {
                return Json(AdmissionResponse::deny());
            };
            path.push(&user.username);
            url.set_path(&path.join("/"));
            Json(AdmissionResponse::allow(url))
        }
        Err(_) => Json(AdmissionResponse::deny()),
    }
}

/// Get the currently logged in user.
pub async fn user(State(db): State<Db>, cookies: Cookies) -> Result<Json<SendableUser>, OMError> {
    let user = User::from_req(State(db), cookies)
        .await
        .map(|u| Json(u.into()))?;
    Ok(user)
}

/// Create a session and set the cookie.
pub async fn login(
    State(db): State<Db>,
    cookies: Cookies,
    Json(creds): Json<UserLogin>,
) -> Result<(), OMError> {
    // Check if a session already exists
    if let Some(c) = cookies.get("om_session") {
        let token = c.value().to_string();
        if (User::from_session(&token, &db).await).is_some() {
            return Ok(());
        }
    }

    let user = User::from_name(&creds.username, &db)
        .await
        .ok_or(OMError::NotFound(creds.username))?;

    spawn_blocking(move || verify_password(&user.password, creds.password.as_bytes())).await??;

    let token = base64::encode(crate::crypto::random_data(64));
    sqlx::query!(
        "INSERT INTO sessions (session, user_id) VALUES(?, ?)",
        token,
        user.username
    )
    .execute(&db)
    .await?;

    let session_cookie = Cookie::build("om_session", token)
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(true)
        .expires(time::OffsetDateTime::now_utc() + time::Duration::days(14))
        .finish();
    cookies.add(session_cookie);

    Ok(())
}

/// Remove the session cookie.
pub async fn logout(State(db): State<Db>, cookies: Cookies) -> Result<(), OMError> {
    let Some(om_cookie) = cookies.get("om_session") else {
        return Ok(());
    };
    let token = om_cookie
        .value()
        .parse::<String>()
        .map_err(|_| OMError::InvalidSession)?;

    sqlx::query!("DELETE FROM sessions WHERE session = ?", token)
        .execute(&db)
        .await?;

    cookies.remove(Cookie::build("om_session", "").path("/").finish());

    Ok(())
}

/// Register a new user, making sure that the username is valid.
pub async fn register(State(db): State<Db>, Json(creds): Json<UserLogin>) -> Result<(), OMError> {
    if User::from_name(&creds.username, &db).await.is_some() {
        return Err(OMError::NameTaken);
    };

    USERNAME_RE
        .is_match(&creds.username)
        .then_some(())
        .ok_or(OMError::InvalidUsername)?;

    let hashed_password =
        tokio::task::spawn_blocking(move || hash_password(creds.password.as_bytes())).await??;

    let stream_key = gen_stream_key();

    sqlx::query!(
        "INSERT INTO users(username, display_name, password, stream_key) VALUES(?, ?, ?, ?)",
        creds.username,
        creds.username,
        hashed_password,
        stream_key
    )
    .execute(&db)
    .await?;

    Ok(())
}

/// Get all users in the database.
pub async fn list_users(
    State(db): State<Db>,
    cookies: Cookies,
) -> Result<Json<Vec<SendableUser>>, OMError> {
    let logged_user = User::from_req(State(db.clone()), cookies).await?;
    if !logged_user.is_admin() {
        return Err(OMError::NoPermission);
    };

    let users: Vec<SendableUser> = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&db)
        .await?
        .into_iter()
        .map(|mut u| {
            u.stream_key = String::new(); // Don't send the stream key
            u.into()
        })
        .collect();

    Ok(Json(users))
}

/// Update a user.
pub async fn update_user(
    State(db): State<Db>,
    cookies: Cookies,
    Json(body): Json<UserUpdate>,
) -> Result<(), OMError> {
    let performing_user = User::from_req(State(db.clone()), cookies).await?;
    if body.username.is_some() && !performing_user.is_admin() {
        return Err(OMError::NoPermission);
    };

    let user = match &body.username {
        Some(u) => User::from_name(u, &db).await,
        None => Some(performing_user.clone()),
    };
    let user = match user {
        Some(u) => u,
        None => return Err(OMError::NotFound(body.username.unwrap_or_default())),
    };

    // check the individual fields
    if let Some(dn) = &body.display_name {
        sqlx::query!(
            "UPDATE users SET display_name = ? WHERE username = ?",
            dn,
            user.username
        )
        .execute(&db)
        .await?;
    };

    if let Some(stream_title) = &body.stream_title {
        sqlx::query!(
            "UPDATE users SET stream_title = ? WHERE username = ?",
            stream_title,
            user.username
        )
        .execute(&db)
        .await?;
    };

    let new_password: Option<String> = match (body.old_password.clone(), body.new_password.clone())
    {
        (None, Some(np)) => {
            if performing_user.is_admin() {
                hash_password(np.as_bytes()).ok()
            } else {
                return Err(OMError::NoPermission);
            }
        }
        (Some(op), Some(np)) => {
            if verify_password(&user.password, op.as_bytes()).is_ok() {
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
        .execute(&db)
        .await?;
    };

    if performing_user.is_admin() {
        if let Some(permissions) = &body.permissions {
            sqlx::query!(
                "UPDATE users SET permissions = ? WHERE username = ?",
                permissions,
                user.username
            )
            .execute(&db)
            .await?;
        }
    } else if body.permissions.is_some() {
        return Err(OMError::NoPermission);
    };

    Ok(())
}

/// Get all currently active streams.
pub async fn streams(
    State(db): State<Db>,
    State(config): State<OMConfig>,
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
        match User::from_name(&s, &db).await {
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

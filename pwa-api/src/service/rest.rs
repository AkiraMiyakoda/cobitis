// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use anyhow::anyhow;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_session_sqlx::SessionPgPool;
use secure_string::SecureString;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::utils;

type Session = axum_session::Session<SessionPgPool>;

#[derive(Debug, Deserialize)]
pub struct Credentials {
    username: SecureString,
    password: SecureString,
}

#[derive(Debug, Serialize)]
pub struct AuthSuccess {}

impl IntoResponse for AuthSuccess {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct AccessToken {
    token: Uuid,
}

impl IntoResponse for AccessToken {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct AuthError {
    reason: Option<String>,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, Json(self)).into_response()
    }
}

// const TOKEN_TTL: &str = "3 MINUTES";

pub async fn get_is_authed(session: Session) -> anyhow::Result<AuthSuccess, AuthError> {
    // Return OK if the current session is authenticated, otherwise return UNAUTHORIZED
    match session.get::<Uuid>("user_id") {
        Some(_) => Ok(AuthSuccess {}),
        None => Err(AuthError { reason: None }),
    }
}

pub async fn post_login(session: Session, Json(creds): Json<Credentials>) -> anyhow::Result<AuthSuccess, AuthError> {
    // Verify the username and password
    match verify_credentials(creds).await {
        Ok(user_id) => {
            // Renew the session and return OK if the username and password is correct
            session.renew();
            session.set("user_id", user_id);

            Ok(AuthSuccess {})
        }
        Err(e) => {
            // Destroy the session and return UNAUTHORIZED if not correct
            session.destroy();

            Err(AuthError {
                reason: Some(format!("{e:?}")),
            })
        }
    }
}

pub async fn post_logout(session: Session) -> AuthSuccess {
    // Destroy the session
    session.destroy();
    AuthSuccess {}
}

pub async fn get_access_token(session: Session) -> anyhow::Result<AccessToken, AuthError> {
    Err(AuthError {
        reason: Some("".into()),
    })
}

async fn verify_credentials(creds: Credentials) -> anyhow::Result<Uuid> {
    // Find a user associated with the username (= auth_id)
    #[derive(FromRow)]
    struct Row {
        user_id: Uuid,
        auth_hash: String,
    }

    let row: Option<Row> = sqlx::query_as(
        r#"
        SELECT
            user_id,
            auth_hash
        FROM
            users
        WHERE
            auth_id = $1
        ;"#,
    )
    .bind(creds.username.unsecure())
    .fetch_optional(utils::pg::pool().await)
    .await?;

    let (user_id, hash) = match row {
        Some(row) => (row.user_id, row.auth_hash),
        None => return Err(anyhow!("User not found")),
    };

    // Verify the password with the hash
    let password = creds.password.unsecure();
    let hash = PasswordHash::new(&hash).expect("Bad password hash");
    match Argon2::default().verify_password(password.as_bytes(), &hash) {
        Ok(_) => Ok(user_id),
        Err(_) => Err(anyhow!("Bad password")),
    }
}

async fn issue_access_token(user_id: Uuid) -> anyhow::Result<Uuid> {
    Ok(Uuid::nil())
}

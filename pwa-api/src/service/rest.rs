// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use axum_session_sqlx::SessionPgPool;
use secure_string::SecureString;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    Err(AuthError {
        reason: Some("".into()),
    })
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

async fn issue_access_token(user_id: Uuid) -> anyhow::Result<Uuid> {
    Ok(Uuid::nil())
}

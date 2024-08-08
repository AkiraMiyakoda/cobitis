// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use axum::{http::StatusCode, Json};
use axum_session_sqlx::SessionPgPool;

use super::models::rest::*;

type Session = axum_session::Session<SessionPgPool>;

// const TOKEN_TTL: &str = "3 MINUTES";

pub async fn get_is_authed(session: Session) -> anyhow::Result<Json<AuthResult>, StatusCode> {
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn post_login(
    session: Session,
    Json(creds): Json<Credentials>,
) -> anyhow::Result<Json<AuthResult>, StatusCode> {
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn post_logout(session: Session) -> anyhow::Result<(), StatusCode> {
    Err(StatusCode::UNAUTHORIZED)
}

pub async fn get_access_token(session: Session) -> anyhow::Result<Json<AccessToken>, StatusCode> {
    Err(StatusCode::UNAUTHORIZED)
}

// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use anyhow::anyhow;
use axum::{
    routing::{get, post},
    Router,
};
use axum_session::{SameSite, SessionConfig, SessionLayer, SessionStore};
use axum_session_sqlx::SessionPgPool;
use chrono::Duration;
use log::info;
use socketioxide::{SocketIoBuilder, TransportType};
use tokio::net::TcpListener;
use tower_http::catch_panic::CatchPanicLayer;

use crate::utils::pg;

mod rest;
mod socketio;

const REST_V1_PATH: &str = "/api/1";
const SOCKETIO_V1_PATH: &str = "/api/1/socket.io/";
const ENDPOINT: &str = "0.0.0.0:8080";

pub async fn serve() -> anyhow::Result<()> {
    info!("PWA API service started.");

    let socketio_layer = {
        let (layer, io) = SocketIoBuilder::new()
            .req_path(SOCKETIO_V1_PATH)
            .transports([TransportType::Websocket])
            .build_layer();
        io.ns("/", socketio::on_connect);

        layer
    };

    let session_layer = {
        let pool = pg::pool().await;
        let config = SessionConfig::new()
            .with_session_name("SID")
            .with_cookie_path("/")
            .with_http_only(true)
            .with_secure(true)
            .with_cookie_same_site(SameSite::Strict)
            .with_max_age(Some(Duration::days(180)))
            .with_always_save(true);
        let store = SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), config).await?;

        SessionLayer::new(store)
    };

    let listener = TcpListener::bind(ENDPOINT).await?;
    let app = Router::new()
        .layer(CatchPanicLayer::new())
        .nest(
            REST_V1_PATH,
            Router::new()
                .route("/is-authed", get(rest::get_is_authed))
                .route("/login", post(rest::post_login))
                .route("/logout", post(rest::post_logout))
                .route("/access-token", get(rest::get_access_token)),
        )
        .layer(socketio_layer)
        .layer(session_layer);
    axum::serve(listener, app)
        .await
        .map_err(|e| anyhow!("Axum error: {e:?}"))
}

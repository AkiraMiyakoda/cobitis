// Copyright © 2025 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use anyhow::anyhow;
use axum::{Json, Router, http::StatusCode, routing::get};
use tokio::net::TcpListener;

use crate::{
    measurements::{self, Measurements},
    signal::{self, Signal},
};

const ENDPOINT: &str = "0.0.0.0:8888";

pub(crate) async fn worker() -> anyhow::Result<()> {
    let listener = TcpListener::bind(ENDPOINT).await?;
    let app = Router::new()
        .route("/measurements", get(get_measurements))
        .route("/signal", get(get_signal));
    axum::serve(listener, app)
        .await
        .map_err(|e| anyhow!("Axum error: {e:?}"))
}

async fn get_measurements() -> Result<Json<Measurements>, StatusCode> {
    measurements::latest().await.map(Json).ok_or(StatusCode::NO_CONTENT)
}

async fn get_signal() -> Result<Json<Signal>, StatusCode> {
    signal::latest().await.map(Json).ok_or(StatusCode::NO_CONTENT)
}

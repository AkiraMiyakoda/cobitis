// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use anyhow::anyhow;
use axum::Router;
use log::info;
use socketioxide::SocketIoBuilder;
use tokio::net::TcpListener;
use tower_http::catch_panic::CatchPanicLayer;

mod models;
mod socketio;

const SOCKETIO_V1_PATH: &str = "/api/s/1/socket.io/";
const ENDPOINT: &str = "0.0.0.0:8081";

pub async fn serve() -> anyhow::Result<()> {
    info!("Sensor API service started.");

    let socketio_layer = {
        let (layer, io) = SocketIoBuilder::new()
            .req_path(SOCKETIO_V1_PATH)
            .build_layer();
        io.ns("/", socketio::on_connect);

        layer
    };

    let listener = TcpListener::bind(ENDPOINT).await?;
    let app = Router::new()
        .layer(CatchPanicLayer::new())
        .layer(socketio_layer);
    axum::serve(listener, app)
        .await
        .map_err(|e| anyhow!("Axum error: {e:?}"))
}

// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use serde::Deserialize;
use socketioxide::extract::{Data, SocketRef};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Token(Uuid);

#[derive(Debug, Deserialize)]
pub struct Handshake {
    token: Token,
}

pub async fn on_connect(socket: SocketRef, Data(handshake): Data<Handshake>) {}

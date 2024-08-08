// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use socketioxide::extract::{Data, SocketRef};

use super::models::socketio::*;

pub async fn on_connect(socket: SocketRef, Data(handshake): Data<AccessToken>) {}

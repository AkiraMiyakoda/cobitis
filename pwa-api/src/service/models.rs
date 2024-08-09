// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod socketio {
    use serde::Deserialize;
    use uuid::Uuid;

    #[derive(Debug, Deserialize)]
    pub struct Token(pub Uuid);

    #[derive(Debug, Deserialize)]
    pub struct AccessToken {
        pub token: Token,
    }
}

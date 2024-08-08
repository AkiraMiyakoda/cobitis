// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub mod rest {
    use secure_string::SecureString;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Debug, Deserialize)]
    pub struct Credentials {
        pub username: SecureString,
        pub password: SecureString,
    }

    #[derive(Debug, Serialize)]
    pub struct AuthResult {
        pub success: bool,
        pub reason: Option<String>,
    }

    #[derive(Debug, Serialize)]
    pub struct Token(pub Uuid);

    #[derive(Debug, Serialize)]
    pub struct AccessToken {
        pub token: Token,
    }
}

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

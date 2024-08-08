// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use secure_string::SecureString;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct SensorId(pub Uuid);

#[derive(Debug, Deserialize)]
pub struct Handshake {
    pub sensor_id: SensorId,
    pub secret: SecureString,
}

#[derive(Debug, Deserialize)]
pub struct Readings(pub f64, pub f64, pub f64);

// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use anyhow::anyhow;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use log::{error, info};
use socketioxide::extract::{Data, SocketRef};

use crate::utils::pg;

use super::models::*;

pub async fn on_connect(socket: SocketRef, Data(handshake): Data<Handshake>) {
    let sensor_id = match verify_handshake(handshake).await {
        Ok(sensor_id) => {
            info!("Sensor accepted: sid={:?}", socket.id);
            sensor_id
        }
        Err(e) => {
            error!("Sensor rejected: sid={:?}, {:?}", socket.id, e);
            return;
        }
    };

    socket.on_disconnect(|socket: SocketRef| {
        info!("Sensor disconnected: sid={:?}", socket.id);
    });

    socket.on("post", move |Data(readings): Data<Readings>| async move {
        if let Err(e) = write_readings(sensor_id, readings).await {
            error!("Could not write readings: {e:?}");
        }
    });
}

async fn verify_handshake(handshake: Handshake) -> anyhow::Result<SensorId> {
    let row: Option<(String,)> = sqlx::query_as(
        r#"
        SELECT
            auth_hash
        FROM
            sensors
        WHERE
            sensor_id = $1
        ;"#,
    )
    .bind(handshake.sensor_id.0)
    .fetch_optional(pg::pool().await)
    .await?;

    let hash = match row {
        Some((hash,)) => hash,
        None => return Err(anyhow!("Sensor not found")),
    };

    let secret = handshake.secret.unsecure();
    let hash = PasswordHash::new(&hash).expect("Bad secret hash");
    match Argon2::default().verify_password(secret.as_bytes(), &hash) {
        Ok(_) => Ok(handshake.sensor_id),
        Err(_) => Err(anyhow!("Bad API secret")),
    }
}

async fn write_readings(sensor_id: SensorId, readings: Readings) -> anyhow::Result<()> {
    let pool = pg::pool().await;
    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        INSERT INTO readings (
            sensor_id,
            temp_in,
            temp_out,
            tds
        ) VALUES (
            $1,
            $2,
            $3,
            $4
        )
        ON CONFLICT DO NOTHING
        ;"#,
    )
    .bind(sensor_id.0)
    .bind(readings.0)
    .bind(readings.1)
    .bind(readings.2)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

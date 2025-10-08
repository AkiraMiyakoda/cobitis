// Copyright © 2025 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{
    process::Command,
    sync::{Arc, LazyLock},
    time::Duration,
};

use anyhow::anyhow;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use logger::log::error;
use regex::Regex;
use serde::Serialize;
use tokio::{
    sync::RwLock,
    task,
    time::{MissedTickBehavior, interval},
};

#[derive(Debug, Clone, Copy, Serialize)]
pub(crate) struct Signal {
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
    pub quality: f64,
}

impl Signal {
    fn new(level: f64) -> Self {
        Self {
            timestamp: Utc::now(),
            quality: level,
        }
    }
}

static LATEST: LazyLock<RwLock<Option<Signal>>> = LazyLock::new(|| RwLock::new(None));

pub(crate) async fn latest() -> Option<Signal> {
    *LATEST.read().await
}

struct Context {
    rx_quality: Regex,
}

impl Context {
    async fn new() -> anyhow::Result<Arc<Self>> {
        task::spawn_blocking(move || {
            let rx_quality = Regex::new(r"Link Quality=\s*([0-9]+)\s*/\s*([0-9]+)").unwrap();
            Ok(Arc::new(Self { rx_quality }))
        })
        .await?
    }
}

pub(crate) async fn worker() -> anyhow::Result<()> {
    let mut interval = interval(Duration::from_secs(30));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let ctx = Context::new().await?;

    loop {
        interval.tick().await;

        if let Err(e) = update(&ctx).await {
            error!("Failed to update signal level: {e:?}");
        }
    }
}

async fn update(ctx: &Arc<Context>) -> anyhow::Result<()> {
    let signal = read(ctx).await?;
    *LATEST.write().await = Some(signal);

    Ok(())
}

async fn read(ctx: &Arc<Context>) -> anyhow::Result<Signal> {
    let ctx = ctx.clone();
    task::spawn_blocking(move || {
        let output = Command::new("iwconfig").args(["wlan0"]).output()?;
        let raw = String::from_utf8(output.stdout)?;
        let Some(caps) = ctx.rx_quality.captures(&raw) else {
            return Err(anyhow!("Invalid format"));
        };
        let num: i32 = caps[1].parse().unwrap();
        let denom: i32 = caps[2].parse().unwrap();
        let quality = (f64::from(num) / f64::from(denom) * 100.0).round() / 100.0;

        Ok(Signal::new(quality))
    })
    .await?
}

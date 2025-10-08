// Copyright © 2025 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{
    fs,
    path::PathBuf,
    sync::{Arc, LazyLock, Mutex},
    time::Duration,
};

use ads1x1x::{Ads1x1x, FullScaleRange, TargetAddr, channel};
use anyhow::anyhow;
use chrono::{DateTime, Utc, serde::ts_milliseconds};
use linux_embedded_hal::{I2cdev, nb::block};
use logger::log::error;
use regex::Regex;
use serde::Serialize;
use tokio::{
    sync::RwLock,
    task,
    time::{MissedTickBehavior, interval},
};

type Ads1115 = ads1x1x::Ads1x1x<
    linux_embedded_hal::I2cdev,
    ads1x1x::ic::Ads1115,
    ads1x1x::ic::Resolution16Bit,
    ads1x1x::mode::OneShot,
>;

#[derive(Debug, Clone, Copy, Serialize)]
pub(crate) struct Measurements {
    #[serde(with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
    pub temperature: f64,
    pub tds: f64,
}

impl Measurements {
    fn new(temperature: f64, tds: f64) -> Self {
        Self {
            timestamp: Utc::now(),
            temperature,
            tds,
        }
    }
}

static LATEST: LazyLock<RwLock<Option<Measurements>>> = LazyLock::new(|| RwLock::new(None));

pub(crate) async fn latest() -> Option<Measurements> {
    *LATEST.read().await
}

struct Context {
    temperature_path: PathBuf,
    rx_temperature: Regex,
    tds_adc: Mutex<Ads1115>,
}

impl Context {
    async fn new() -> anyhow::Result<Arc<Self>> {
        task::spawn_blocking(move || {
            let temperature_path = {
                let mut dir = fs::read_dir("/sys/bus/w1/devices")?.flatten();
                loop {
                    match dir.next() {
                        Some(entry) => {
                            let path = entry.path().join("w1_slave");
                            if path.is_file() {
                                break path;
                            }
                        }
                        None => return Err(anyhow!("Thermal sensor not found")),
                    }
                }
            };
            let rx_temperature = Regex::new(r"t=\s*([0-9]+)").unwrap();

            let tds_adc = {
                let dev = I2cdev::new("/dev/i2c-1")?;
                let mut adc = Ads1x1x::new_ads1115(dev, TargetAddr::default());
                adc.set_full_scale_range(FullScaleRange::Within4_096V)
                    .map_err(|e| anyhow!("{e:?}"))?;

                Mutex::new(adc)
            };

            Ok(Arc::new(Self {
                temperature_path,
                rx_temperature,
                tds_adc,
            }))
        })
        .await?
    }
}

pub(crate) async fn worker() -> anyhow::Result<()> {
    let mut interval = interval(Duration::from_secs(10));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let ctx = Context::new().await?;

    loop {
        interval.tick().await;

        if let Err(e) = update(&ctx).await {
            error!("Failed to update measurements: {e:?}");
        }
    }
}

async fn update(ctx: &Arc<Context>) -> anyhow::Result<()> {
    let measurements = read(ctx).await?;
    *LATEST.write().await = Some(measurements);

    Ok(())
}

async fn read(ctx: &Arc<Context>) -> anyhow::Result<Measurements> {
    const MAX_VOLTAGE: f64 = 4.096;
    const MAX_RAW_VALUE: f64 = 32767.0;

    let ctx = ctx.clone();
    task::spawn_blocking(move || {
        let temperature = {
            let raw = fs::read_to_string(&ctx.temperature_path)?;
            let Some(caps) = ctx.rx_temperature.captures(&raw) else {
                return Err(anyhow!("Invalid format"));
            };
            let millis: i32 = caps[1].parse().unwrap();

            (f64::from(millis) / 100.0).round() / 10.0
        };

        let tds = {
            let mut adc = ctx.tds_adc.lock().map_err(|e| anyhow!("{e:?}"))?;
            let raw_value = block!(adc.read(channel::SingleA0)).map_err(|e| anyhow!("{e:?}"))?;
            let voltage = f64::from(raw_value) * MAX_VOLTAGE / MAX_RAW_VALUE;

            let coefficient = 1.0 + 0.02 * (temperature - 25.0);
            let voltage = voltage / coefficient;
            let tds = (133.42 * voltage.powf(3.0) - 255.86 * voltage.powf(2.0) + 857.39 * voltage) * 0.5;

            (tds * 10.0).round() / 10.0
        };

        Ok(Measurements::new(temperature, tds))
    })
    .await?
}

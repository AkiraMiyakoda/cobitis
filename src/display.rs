// Copyright © 2025 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::anyhow;
use chrono::Local;
use eg_bdf::BdfTextStyle;
use eg_font_converter::{EgBdfOutput, FontConverter, Mapping};
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyleBuilder},
    text::{Baseline, Text},
};
use linux_embedded_hal::I2cdev;
use logger::log::error;
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*, size::DisplaySize128x64};
use tokio::{
    task,
    time::{MissedTickBehavior, interval},
};

use crate::{measurements, signal};

type Display = Ssd1306<
    I2CInterface<linux_embedded_hal::I2cdev>,
    DisplaySize128x64,
    ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
>;

struct Context {
    display: Mutex<Display>,
    fonts: (EgBdfOutput, EgBdfOutput),
}

impl Context {
    async fn new() -> anyhow::Result<Arc<Self>> {
        task::spawn_blocking(move || {
            let display = {
                let iwc = I2cdev::new("/dev/i2c-1")?;
                let iface = I2CDisplayInterface::new(iwc);
                let mut display =
                    Ssd1306::new(iface, DisplaySize128x64, DisplayRotation::Rotate0).into_buffered_graphics_mode();
                display.init().map_err(|e| anyhow!("{e:?}"))?;
                display.clear_buffer();
                display.flush().map_err(|e| anyhow!("{e:?}"))?;

                Mutex::new(display)
            };

            let fonts = (
                FontConverter::with_string(include_str!("../fonts/ter-u14b.bdf"), "ter_u14b")
                    .glyphs(Mapping::Iso8859_1)
                    .missing_glyph_substitute('?')
                    .convert_eg_bdf()
                    .unwrap(),
                FontConverter::with_string(include_str!("../fonts/ter-u24b.bdf"), "ter_u24b")
                    .glyphs(Mapping::Iso8859_1)
                    .missing_glyph_substitute('?')
                    .convert_eg_bdf()
                    .unwrap(),
            );

            Ok(Arc::new(Self { display, fonts }))
        })
        .await?
    }
}

pub(crate) async fn worker() -> anyhow::Result<()> {
    let mut interval = interval(Duration::from_secs(1));
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let ctx = Context::new().await?;

    loop {
        interval.tick().await;

        if let Err(e) = draw(&ctx).await {
            error!("Failed to update measurements: {e:?}");
        }
    }
}

async fn draw(ctx: &Arc<Context>) -> anyhow::Result<()> {
    let signal = signal::latest().await;
    let measurements = measurements::latest().await;

    let ctx = ctx.clone();
    task::spawn_blocking(move || {
        let mut display = ctx.display.lock().map_err(|e| anyhow!("{e:?}"))?;
        display.clear_buffer();

        let font_refs = (ctx.fonts.0.as_font(), ctx.fonts.1.as_font());
        let text_styles = (
            BdfTextStyle::new(&font_refs.0, BinaryColor::On),
            BdfTextStyle::new(&font_refs.1, BinaryColor::On),
        );
        let line_style = PrimitiveStyleBuilder::new()
            .stroke_width(1)
            .stroke_color(BinaryColor::On)
            .build();

        // Draw current datetime
        let datetime = Local::now().format("%m·%d %H:%M").to_string();
        Text::with_baseline(&datetime, Point::new(10, 0), text_styles.0, Baseline::Top)
            .draw(&mut *display)
            .unwrap();

        // Draw signal level
        if let Some(signal) = signal {
            assert!(signal.quality.is_finite() && signal.quality <= 1.0);

            let level = match signal.quality {
                q if q < 0.2 => 0,
                q if q < 0.4 => 1,
                q if q < 0.6 => 2,
                q if q < 0.8 => 3,
                _ => 4,
            };
            for i in 1..=level {
                let x = 107 + i * 2;
                let y = 12 - i * 2;
                Line::new(Point::new(x, y), Point::new(x, 11))
                    .into_styled(line_style)
                    .draw(&mut *display)
                    .unwrap();
            }
        }

        // Draw temperature
        let temp: Cow<_> = if let Some(v) = measurements.map(|m| m.temperature) {
            format!("{v:>7.1}").into()
        } else {
            "    -.-".into()
        };

        Text::with_baseline(&temp, Point::new(0, 16), text_styles.1, Baseline::Top)
            .draw(&mut *display)
            .unwrap();
        Text::with_baseline(&temp, Point::new(1, 16), text_styles.1, Baseline::Top)
            .draw(&mut *display)
            .unwrap();
        Text::with_baseline("°C", Point::new(89, 23), text_styles.0, Baseline::Top)
            .draw(&mut *display)
            .unwrap();

        // Draw TDS
        let tds: Cow<_> = if let Some(v) = measurements.map(|m| m.tds) {
            format!("{v:>7.0}").into()
        } else {
            "      -".into()
        };

        Text::with_baseline(&tds, Point::new(0, 40), text_styles.1, Baseline::Top)
            .draw(&mut *display)
            .unwrap();
        Text::with_baseline(&tds, Point::new(1, 40), text_styles.1, Baseline::Top)
            .draw(&mut *display)
            .unwrap();
        Text::with_baseline("ppm", Point::new(90, 47), text_styles.0, Baseline::Top)
            .draw(&mut *display)
            .unwrap();

        display.flush().map_err(|e| anyhow!("{e:?}"))?;

        Ok(())
    })
    .await?
}

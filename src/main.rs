// Copyright © 2025 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use logger::log::info;
use tokio::select;

mod api;
mod display;
mod measurements;
mod signal;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    logger::init();

    info!("Cobitis: tank monitor service started");

    select! {
        result = measurements::worker() => result,
        result = signal::worker() => result,
        result = api::worker() => result,
        result = display::worker() => result,
    }
}

// Copyright © 2024 Akira Miyakoda
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use log::error;
use logger::init_logger;

mod service;
mod utils;

#[tokio::main]
async fn main() {
    init_logger();

    if let Err(e) = service::serve().await {
        error!("{e:?}");
        std::process::exit(1);
    }
}

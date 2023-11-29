// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::app::App;

mod app;
mod auth;
mod cli;
mod geolocation;
mod tui;
mod weatherkit;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    App::run().await
}

// SPDX-License-Identifier: MIT OR Apache-2.0

use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, ValueEnum, Debug)]
pub enum Forecast {
    /// Get the current forecast
    Current,
    /// Get the 10-day forecast
    Weekly,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The forecast to display
    #[arg(short, long, value_enum, default_value_t = Forecast::Current)]
    pub forecast: Forecast,
}

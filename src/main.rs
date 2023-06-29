use crate::app::App;
use crate::cli::Cli;
use clap::Parser;

use crate::weatherkit::DataSet;

mod app;
mod auth;
mod cli;
mod geolocation;
mod weatherkit;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let app = App::new(cli.location).await?;

    let datasets = app.get_available_datasets().await?;
    let weather = app.get_weather(&datasets).await?;
    if datasets.contains(&DataSet::CurrentWeather) {
        println!("{:?}", weather.current_weather)
    }
    Ok(())
}

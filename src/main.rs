use crate::app::App;

use crate::weatherkit::DataSet;

mod app;
mod auth;
mod geolocation;
mod weatherkit;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = App::new().await?;

    let datasets = app.get_available_datasets().await?;
    let weather = app.get_weather(&datasets).await?;
    if datasets.contains(&DataSet::CurrentWeather) {
        println!("{:?}", weather.current_weather)
    }
    Ok(())
}

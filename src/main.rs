use crate::geolocation::Location;
use crate::weatherkit::{CurrentWeather, DataSet, Weather};
use anyhow::{anyhow, Context};
use reqwest::{Client, Method};

mod auth;
mod geolocation;
mod weatherkit;

const WEATHERKIT_API_BASE_URL: &str = "https://weatherkit.apple.com/api/v1";

fn get_availability_url(location: &Location) -> String {
    [
        WEATHERKIT_API_BASE_URL,
        "availability",
        &location.lat,
        &location.lon,
    ]
    .join("/")
}

fn get_weather_url(location: &Location) -> String {
    [
        WEATHERKIT_API_BASE_URL,
        "weather",
        "en",
        &location.lat,
        &location.lon,
    ]
    .join("/")
}

async fn get_available_datasets(
    client: &Client,
    location: &Location,
    auth_token: &str,
) -> reqwest::Result<Vec<weatherkit::DataSet>> {
    let availability_url = get_availability_url(location);
    let request = client
        .request(Method::GET, availability_url)
        .query(&[("country", &location.country_code)])
        .bearer_auth(auth_token)
        .build()?;
    client.execute(request).await?.json::<Vec<DataSet>>().await
}

async fn get_weather(
    client: &Client,
    location: &Location,
    auth_token: &str,
    datasets: &Vec<DataSet>,
) -> anyhow::Result<Weather> {
    let url = get_weather_url(location);
    let mut queries = Vec::from([
        ("countryCode", location.country_code.clone()),
        ("timezone", location.timezone.clone()),
    ]);
    queries.reserve(datasets.len());
    for dataset in datasets {
        queries.push(("dataSets", dataset.to_string()));
    }

    let request = client
        .request(Method::GET, url)
        .query(&queries)
        .bearer_auth(auth_token)
        .build()
        .context("failed to build request")?;

    let weather = client.execute(request).await?.json::<Weather>().await?;
    Ok(weather)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::builder().build()?;
    let auth_token = auth::generate_token()?;
    //let location = get_current_location(&client).await?;
    let location = Location {
        status: "".to_string(),
        country: "United Kingdom".to_owned(),
        country_code: "GB".to_owned(),
        city: "Canterbury".to_owned(),
        lat: "51.279".to_owned(),
        lon: "1.0763".to_owned(),
        timezone: "Europe/London".to_owned(),
    };

    let datasets = get_available_datasets(&client, &location, &auth_token).await?;
    let weather = get_weather(&client, &location, &auth_token, &datasets).await?;
    if datasets.contains(&DataSet::CurrentWeather) {
        println!("{:?}", weather.current_weather)
    }
    Ok(())
}

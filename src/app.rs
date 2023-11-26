use crate::auth::generate_token;
use crate::cli::Cli;
use crate::geolocation::{get_current_location, Location};
use crate::weatherkit::{DataSet, Weather, WEATHERKIT_API_BASE_URL};
use anyhow::{anyhow, Context};
use clap::Parser;
use reqwest::{Client, Method};
use std::borrow::Cow;

pub struct App {
    pub client: Client,
    pub location: Location,
    pub auth_token: String,
}

impl App {
    pub async fn run() -> anyhow::Result<()> {
        let cli = Cli::parse();
        let app = Self::new(cli.location).await?;
        let datasets = app.get_available_datasets().await?;
        let weather = app.get_weather(&datasets).await?;
        match weather.current_weather {
            Some(cw) => {
                println!(
                    "Current weather at {}, {} as of {}.",
                    app.location.city,
                    app.location.country,
                    cw.as_of.format("%H:%M on %B %e %Y")
                );
                println!();
                println!("{}", cw)
            }
            None => {
                return Err(anyhow!(
                    "Current weather for location {}, {} was requested but is not available!",
                    app.location.city,
                    app.location.country_code
                ))
            }
        }
        Ok(())
    }

    pub async fn new(location_str: Option<String>) -> anyhow::Result<Self> {
        let client = Client::builder().build()?;
        // TODO: Add forward geocoding support (so user can supply their own location data
        let location = get_current_location(&client).await?;
        let auth_token = generate_token()?;

        Ok(Self {
            client,
            location,
            auth_token,
        })
    }

    pub async fn get_available_datasets(&self) -> reqwest::Result<Vec<DataSet>> {
        let availability_url = get_availability_url(&self.location);
        let request = self
            .client
            .request(Method::GET, availability_url)
            .query(&[("country", &self.location.country_code)])
            .bearer_auth(&self.auth_token)
            .build()?;
        self.client
            .execute(request)
            .await?
            .json::<Vec<DataSet>>()
            .await
    }

    pub async fn get_weather(&self, datasets: &Vec<DataSet>) -> anyhow::Result<Weather> {
        let url = get_weather_url(&self.location);
        let mut queries = Vec::from([
            ("countryCode", Cow::from(&self.location.country_code)),
            ("timezone", Cow::from(&self.location.timezone)),
        ]);
        queries.reserve(datasets.len());
        for dataset in datasets {
            queries.push(("dataSets", Cow::from(dataset.to_string())));
        }

        let request = self
            .client
            .request(Method::GET, url)
            .query(&queries)
            .bearer_auth(&self.auth_token)
            .build()
            .context("failed to build request")?;

        let weather = self
            .client
            .execute(request)
            .await?
            .json::<Weather>()
            .await?;
        Ok(weather)
    }
}

fn get_availability_url(location: &Location) -> String {
    format!(
        "{}/availability/{}/{}",
        WEATHERKIT_API_BASE_URL, location.lat, location.lon
    )
}

fn get_weather_url(location: &Location) -> String {
    format!(
        "{}/weather/en/{}/{}",
        WEATHERKIT_API_BASE_URL, &location.lat, &location.lon
    )
}

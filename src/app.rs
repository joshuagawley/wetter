use crate::auth::generate_token;
use crate::cli::Cli;
use crate::geolocation::Location;
use crate::weatherkit::{DataSet, Weather};
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
        let datasets = app
            .get_available_datasets()
            .await
            .context("Could not download datasets.")?;
        let weather = app
            .get_weather(&datasets)
            .await
            .context("Could not download weather data.")?;
        match weather.current_weather {
            Some(cw) => cw.prepare(&app.location)?.render(),
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
        let location = Location::get_current_location(&client).await?;
        let auth_token = generate_token()?;

        Ok(Self {
            client,
            location,
            auth_token,
        })
    }

    pub async fn get_available_datasets(&self) -> anyhow::Result<Vec<DataSet>> {
        let availability_url = self.location.get_availability_url();
        let request = self
            .client
            .request(Method::GET, availability_url)
            .query(&[("country", &self.location.country_code)])
            .bearer_auth(&self.auth_token)
            .build()
            .context("Failed to build request")?;
        Ok(self
            .client
            .execute(request)
            .await
            .context("Failed to execute request")?
            .json::<Vec<DataSet>>()
            .await?)
    }

    pub async fn get_weather(&self, datasets: &Vec<DataSet>) -> anyhow::Result<Weather> {
        let weather_url = self.location.get_weather_url();
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
            .request(Method::GET, weather_url)
            .query(&queries)
            .bearer_auth(&self.auth_token)
            .build()
            .context("Failed to build request")?;

        let weather = self
            .client
            .execute(request)
            .await
            .context("Failed to execute request")?
            .json::<Weather>()
            .await?;
        Ok(weather)
    }
}

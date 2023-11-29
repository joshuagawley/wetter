// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::auth::generate_token;
use crate::cli::{Cli, Forecast};
use crate::geolocation::Location;
use crate::weatherkit::{DataSet, Weather};
use anyhow::{anyhow, Context};
use clap::Parser;
use reqwest::{Client, Method};
use std::borrow::Cow;

pub struct App {
    client: Client,
    location: Location,
    auth_token: String,
}

impl App {
    pub async fn run() -> anyhow::Result<()> {
        let cli = Cli::parse();
        let app = Self::new().await?;
        let datasets = app
            .get_available_datasets()
            .await
            .context("Could not download datasets.")?;
        let weather = app
            .get_weather(&datasets)
            .await
            .context("Could not download weather data.")?;

        match cli.forecast {
            Forecast::Current => app.handle_current_forecast(weather),
            Forecast::Weekly => app.handle_weekly_forecast(weather),
            Forecast::Hourly => app.handle_hourly_forecast(weather),
            Forecast::NextHour => app.handle_next_hour(weather),
            Forecast::Alerts => app.handle_alerts(weather),
        }
    }

    pub async fn new() -> anyhow::Result<Self> {
        let client = Client::builder().build()?;
        let location = Location::get_current_location(&client).await?;
        let auth_token = generate_token()?;

        Ok(Self {
            client,
            location,
            auth_token,
        })
    }

    fn handle_current_forecast(&self, weather: Weather) -> anyhow::Result<()> {
        match weather.current_weather {
            Some(cw) => {
                if let Some(daily_forecast) = &weather.forecast_daily {
                    let todays_forecast = &daily_forecast.days[0];

                    if let (Some(sunrise), Some(sunset)) =
                        (todays_forecast.sunrise, todays_forecast.sunset)
                    {
                        cw.prepare(&self.location, &sunrise, &sunset)?.render();
                        Ok(())
                    } else {
                        Err(anyhow!(
                        "Current weather for location {}, {} was requested but is not available!",
                        self.location.city,
                        self.location.country_code
                    ))
                    }
                } else {
                    Err(anyhow!(
                        "Current weather for location {}, {} was requested but is not available!",
                        self.location.city,
                        self.location.country_code
                    ))
                }
            }
            None => Err(anyhow!(
                "Current weather for location {}, {} was requested but is not available!",
                self.location.city,
                self.location.country_code
            )),
        }
    }

    fn handle_weekly_forecast(&self, weather: Weather) -> anyhow::Result<()> {
        match weather.forecast_daily {
            Some(fd) => {
                fd.prepare()?.render();
                Ok(())
            }
            None => Err(anyhow!(
                "Weekly weather for location {}, {} was requested but is not available!",
                self.location.city,
                self.location.country_code
            )),
        }
    }

    fn handle_hourly_forecast(&self, weather: Weather) -> anyhow::Result<()> {
        match weather.forecast_hourly {
            Some(fh) => {
                fh.prepare(&self.location).render();
                Ok(())
            }
            None => Err(anyhow!(
                "Hourly weather for location {}, {} was requested but is not available!",
                self.location.city,
                self.location.country_code
            )),
        }
    }

    fn handle_next_hour(&self, weather: Weather) -> anyhow::Result<()> {
        match weather.forecast_next_hour {
            Some(fnh) => {
                fnh.prepare(&self.location).render();
                Ok(())
            }
            None => Err(anyhow!(
                "Next hour weather for location {}, {} was requested but is not available!",
                self.location.city,
                self.location.country_code
            )),
        }
    }

    fn handle_alerts(&self, weather: Weather) -> anyhow::Result<()> {
        match weather.weather_alerts {
            Some(alerts) => alerts.prepare(&self.location).render(),
            None => println!(
                "No weather alerts at {}, {}",
                self.location.city, self.location.country_code
            ),
        }

        Ok(())
    }

    async fn get_available_datasets(&self) -> anyhow::Result<Vec<DataSet>> {
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

    async fn get_weather(&self, datasets: &Vec<DataSet>) -> anyhow::Result<Weather> {
        let weather_url = self.location.get_weather_url();
        let mut queries = vec![
            ("countryCode", Cow::from(&self.location.country_code)),
            ("timezone", Cow::from(&self.location.timezone)),
        ];
        queries.reserve(datasets.len());
        queries.extend(datasets.iter().map(|x| ("dataSets", x.to_string().into())));

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

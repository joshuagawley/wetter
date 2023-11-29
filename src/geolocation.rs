// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::weatherkit::WEATHERKIT_API_BASE_URL;
use anyhow::Result;
use reqwest::{Client, Method};
use serde::Deserialize;

const IP_API_URL_BASE_PATH: &str = "http://ip-api.com/json/";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub status: String,
    pub country: String,
    pub country_code: String,
    pub city: String,
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
}

impl Location {
    // 16851 gets us fields: status,country,countryCode,city,lat,lon,timezone
    pub async fn get_current_location(client: &Client) -> Result<Self, reqwest::Error> {
        let request = client
            .request(Method::GET, IP_API_URL_BASE_PATH)
            .query(&[("fields", 16851)])
            .build()?;

        client.execute(request).await?.json::<Location>().await
    }

    pub fn get_availability_url(&self) -> String {
        format!(
            "{}/availability/{}/{}",
            WEATHERKIT_API_BASE_URL, self.lat, self.lon
        )
    }

    pub fn get_weather_url(&self) -> String {
        format!(
            "{}/weather/en/{}/{}",
            WEATHERKIT_API_BASE_URL, self.lat, self.lon
        )
    }
}

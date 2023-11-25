use anyhow::Result;
use chrono::TimeZone;
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

// 16851 gets us fields: status,country,countryCode,city,lat,lon,timezone
pub async fn get_current_location(client: &Client) -> Result<Location, reqwest::Error> {
    let request = client
        .request(Method::GET, IP_API_URL_BASE_PATH)
        .query(&[("fields", 16851)])
        .build()?;

    client.execute(request).await?.json::<Location>().await
}

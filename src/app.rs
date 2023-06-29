use crate::auth::generate_token;
use crate::geolocation::Location;
use crate::weatherkit::{DataSet, Weather, WEATHERKIT_API_BASE_URL};
use anyhow::Context;
use reqwest::{Client, Method};

pub struct App {
    pub client: Client,
    pub location: Location,
    pub auth_token: String,
}

impl App {
    pub async fn new() -> anyhow::Result<Self> {
        let client = Client::builder().build()?;
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
            ("countryCode", self.location.country_code.clone()),
            ("timezone", self.location.timezone.clone()),
        ]);
        queries.reserve(datasets.len());
        for dataset in datasets {
            queries.push(("dataSets", dataset.to_string()));
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

use crate::geolocation::get_current_location;
use reqwest::{Client, Method};

mod auth;
mod geolocation;

const WEATHERKIT_API_BASE_URL: &str = "https://weatherkit.apple.com/api/v1";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::builder().build()?;
    let location = get_current_location(&client).await?;
    let url = [
        WEATHERKIT_API_BASE_URL,
        "availability",
        &location.lat.to_string(),
        &location.lon.to_string(),
    ]
    .join("/");

    let token = auth::generate_token()?;
    let request = client
        .request(Method::GET, url)
        .query(&[("country", location.country_code)])
        .bearer_auth(&token)
        .build()?;
    let response = client.execute(request).await?.text().await?;
    println!("{}", response);

    Ok(())
}

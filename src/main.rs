use reqwest::{Client, Method};

mod auth;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::builder().build()?;

    let token = auth::generate_token()?;
    let request = client
        .request(
            Method::GET,
            "https://weatherkit.apple.com/api/v1/availability/51.0/0.0",
        )
        .query(&[("country", "GB")])
        .bearer_auth(&token)
        .build()?;
    let response = client.execute(request).await?.text().await?;
    println!("{}", response);

    Ok(())
}

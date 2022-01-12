use iracing_api::IracingApiClient;

use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv::dotenv()?;

    let email = std::env::var("IRACING_EMAIL").unwrap();
    let password = std::env::var("IRACING_PASSWORD").unwrap();

    let client = IracingApiClient::new(&email, &password).await?;
    println!("Logged in!");

    let response = client.all_car_assets().await?;
    dbg!(response);

    Ok(())
}

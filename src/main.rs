use iracing_api::{
    model::season_results::{EventType, SeasonResultsQuery},
    IracingApiClient,
};

use color_eyre::eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv::dotenv()?;

    let email = std::env::var("IRACING_EMAIL").unwrap();
    let password = std::env::var("IRACING_PASSWORD").unwrap();

    let client = IracingApiClient::new(&email, &password).await?;
    println!("Logged in!");

    let response = client
        .season_results(
            SeasonResultsQuery::new(2345)
                .event_type(EventType::Race)
                .race_week_num(5),
        )
        .await?;

    dbg!(response.results_list.len());

    Ok(())
}

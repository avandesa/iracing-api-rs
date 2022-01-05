use iracing_api::{model::season_results::EventType, IracingApiClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let email = std::env::var("IRACING_EMAIL").unwrap();
    let password = std::env::var("IRACING_PASSWORD").unwrap();

    let client = IracingApiClient::new(&email, &password).await?;
    let response = client
        .season_results(
            iracing_api::client::SeasonResultsQuery::new(2345)
                .event_type(EventType::Race)
                .race_week_num(5),
        )
        .await?;

    println!("{:#?}", response);

    Ok(())
}

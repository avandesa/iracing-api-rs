use crate::{
    auth::{AuthRequestBody, AuthResponseBody},
    model::*,
    Result,
};
use {reqwest::Client, serde::Deserialize};

#[derive(Deserialize, Debug, Clone)]
struct LinkResponseBody {
    link: String,
}

pub struct IracingApiClient {
    reqwest: Client,
    _auth: AuthResponseBody,
}

impl IracingApiClient {
    pub async fn new(email: &str, password: &str) -> Result<Self> {
        let reqwest = Client::builder().cookie_store(true).build()?;
        let auth = reqwest
            .post("https://members-ng.iracing.com/auth")
            .json(&AuthRequestBody {
                email: email.to_string(),
                password: password.to_string(),
            })
            .send()
            .await?
            .json::<AuthResponseBody>()
            .await?;
        Ok(Self {
            reqwest,
            _auth: auth,
        })
    }

    pub async fn session_results(
        &self,
        subsession_id: u32,
        include_licenses: bool,
    ) -> Result<results::SessionResult> {
        let response: LinkResponseBody = self
            .reqwest
            .get("https://members-ng.iracing.com/data/results/get")
            .query(&[
                ("subsession_id", subsession_id.to_string()),
                ("include_licenses", include_licenses.to_string()),
            ])
            .send()
            .await?
            .json()
            .await?;

        let data = self.reqwest.get(response.link).send().await?.json().await?;

        Ok(data)
    }

    pub async fn season_results(
        &self,
        SeasonResultsQuery {
            season_id,
            event_type,
            race_week_num,
        }: SeasonResultsQuery,
    ) -> Result<season_results::SeasonResults> {
        let mut query = vec![("season_id", season_id.to_string())];
        if let Some(event_type) = event_type {
            query.push(("event_type", event_type.to_string()));
        }
        if let Some(race_week_num) = race_week_num {
            query.push(("race_week_num", race_week_num.to_string()));
        }
        let response: LinkResponseBody = self
            .reqwest
            .get("https://members-ng.iracing.com/data/results/season_results")
            .query(&query)
            .send()
            .await?
            .json()
            .await?;

        let data = self.reqwest.get(response.link).send().await?.json().await?;

        Ok(data)
    }
}

pub struct SeasonResultsQuery {
    season_id: u32,
    event_type: Option<season_results::EventType>,
    race_week_num: Option<u32>,
}

impl SeasonResultsQuery {
    pub fn new(season_id: u32) -> Self {
        Self {
            season_id,
            event_type: None,
            race_week_num: None,
        }
    }

    pub fn event_type(mut self, event_type: season_results::EventType) -> Self {
        self.event_type = Some(event_type);
        self
    }

    pub fn race_week_num(mut self, race_week_num: u32) -> Self {
        self.race_week_num = Some(race_week_num);
        self
    }
}

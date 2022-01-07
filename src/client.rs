use crate::model::{auth::*, *};
use {color_eyre::eyre::Result, reqwest::Client, serde::Deserialize, thiserror::Error};

#[derive(Deserialize, Debug, Clone)]
struct LinkResponseBody {
    link: String,
}

/// An authenticated iRacing API client
pub struct IracingApiClient {
    reqwest: Client,
    /// The data returned by iRacing after authentication
    pub auth: AuthSuccessBody,
}

impl IracingApiClient {
    /// Create a new iRacing API client and authenticate with the iRacing service.
    ///
    /// # Errors
    ///
    /// This method fails if a TLS backend cannot be initialized, or the resolver cannot load the system configuration.
    ///
    /// This method may also fail if one of the following issues occurs during authentication
    ///
    /// - The provided email & password are invalid
    /// - The provided email is malformed
    /// - iRacing decides to require manual login verification for the user
    ///
    /// See the documentation of [AuthError] for more details
    ///
    /// # Panics
    ///
    /// Panics if the HTTP response from iRacing is malformed
    pub async fn new(email: &str, password: &str) -> Result<Self, ClientInitError> {
        // Initialize a reqwest client with a cookie store enabled
        let reqwest = Client::builder().cookie_store(true).build()?;

        // Attempt to authenticate with iRacing
        let auth_response = reqwest
            .post("https://members-ng.iracing.com/auth")
            .json(&AuthRequestBody {
                email: email.to_string(),
                password: password.to_string(),
            })
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        match AuthResponse::from_json(auth_response) {
            AuthResponse::Success(auth) => Ok(IracingApiClient { reqwest, auth }),
            AuthResponse::Failure(body) => {
                // Match on the error message returned from iRacing
                let err_kind = match body.message.as_str() {
                    "Invalid email address or password. Please try again." => {
                        AuthErrorKind::InvalidCredentials
                    }
                    "Missing auth identifier." => AuthErrorKind::MissingAuthIdentifier,
                    "Verification required." => AuthErrorKind::VerificationRequired,
                    other => {
                        // This should be considered a bug
                        AuthErrorKind::Unknown(format!("Unknown auth failure message: {}", other))
                    }
                };
                Err(ClientInitError::AuthenticationFailure(AuthError {
                    kind: err_kind,
                    body,
                }))
            }
        }
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

#[derive(Error, Debug)]
pub enum ClientInitError {
    #[error("Cannot initialize HTTP client")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Authentication with iRacing failed")]
    AuthenticationFailure(#[from] AuthError),
}

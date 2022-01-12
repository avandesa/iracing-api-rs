use crate::model::{auth::*, *};
use {eyre::Result, reqwest::Client, serde::Deserialize, thiserror::Error};

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
        let reqwest = Client::builder()
            .cookie_store(true)
            .build()
            .map_err(ClientInitError::ReqwestInitError)?;

        // Attempt to authenticate with iRacing
        let auth_response = reqwest
            .post("https://members-ng.iracing.com/auth")
            .json(&AuthRequestBody {
                email: email.to_string(),
                password: password.to_string(),
            })
            .send()
            .await
            .map_err(ClientInitError::ConnectionFailure)?
            .json::<serde_json::Value>()
            .await
            .map_err(ClientInitError::ConnectionFailure)?;

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
        query: season_results::SeasonResultsQuery,
    ) -> Result<season_results::SeasonResults> {
        let response: LinkResponseBody = self
            .reqwest
            .get("https://members-ng.iracing.com/data/results/season_results")
            .query(&query.as_query_params())
            .send()
            .await?
            .json()
            .await?;

        let data = self.reqwest.get(response.link).send().await?.json().await?;

        Ok(data)
    }

    pub async fn all_cars(&self) -> Result<Vec<car::Car>> {
        let response: LinkResponseBody = self
            .reqwest
            .get("https://members-ng.iracing.com/data/car/get")
            .send()
            .await?
            .json()
            .await?;

        let data = self.reqwest.get(response.link).send().await?.json().await?;

        Ok(data)
    }
}

#[derive(Error, Debug)]
pub enum ClientInitError {
    #[error("Cannot initialize HTTP client")]
    ReqwestInitError(reqwest::Error),
    #[error("Connection failure")]
    ConnectionFailure(reqwest::Error),
    #[error("Authentication with iRacing failed")]
    AuthenticationFailure(#[from] AuthError),
}

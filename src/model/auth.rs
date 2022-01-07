use {
    serde::{Deserialize, Serialize},
    thiserror::Error,
};

#[derive(Serialize, Debug, Clone)]
pub struct AuthRequestBody {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthResponseBody {
    #[serde(rename = "authcode")]
    pub auth_code: String,
    #[serde(rename = "autoLoginSeries")]
    pub auto_login_series: Option<String>,
    #[serde(rename = "autoLoginToken")]
    pub auto_login_token: Option<String>,

    #[serde(rename = "custId")]
    pub cust_id: u32,
    pub email: String,

    #[serde(rename = "ssoCookieDomain")]
    pub sso_cookie_domain: String,
    #[serde(rename = "ssoCookieName")]
    pub sso_cookie_name: String,
    #[serde(rename = "ssoCookiePath")]
    pub sso_cookie_path: String,
    #[serde(rename = "ssoCookieValue")]
    pub sso_cookie_value: String,
}

/// The various ways authentication can fail
#[derive(Error, Debug, Clone)]
pub enum AuthError {
    /// The provided username or password (or both!) was incorrect
    #[error("Invalid email address or password")]
    InvalidCredentials,

    /// Usually caused by a malformed email, check the format
    #[error("Missing auth identifier")]
    MissingAuthIdentifier,

    /// Usually caused by too many failed login attempts. iRacing is requiring
    /// manual verification. Go to <https://members-login.iracing.com> and
    /// re-enter your login credentials.  See [this forum post][post] for more
    /// information.
    /// 
    /// [post]: https://forums.iracing.com/discussion/comment/113257/#Comment_113257
    #[error("Verification required")]
    VerificationRequired,

    /// Authentication failed and returned a repsonse in the correct format, but
    /// the error message was not recognized
    #[error("Unkown authentication error: {0}")]
    Unknown(String),
}

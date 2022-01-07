use std::fmt;

use {
    serde::{Deserialize, Serialize},
    thiserror::Error,
};

#[derive(Serialize, Debug, Clone)]
pub struct AuthRequestBody {
    pub email: String,
    pub password: String,
}

pub enum AuthResponse {
    Success(AuthSuccessBody),
    Failure(AuthFailureBody),
}

impl AuthResponse {
    pub fn from_json(value: serde_json::Value) -> Self {
        match value
            .get("authcode")
            .expect("Field \"authcode\" missing from response body")
        {
            serde_json::Value::String(_) => Self::Success(
                serde_json::from_value(value)
                    .expect("Could not interpret json value as AuthSuccessBody"),
            ),
            serde_json::Value::Number(_) => Self::Failure(
                serde_json::from_value(value)
                    .expect("Could not interpret json value as AuthSuccessBody"),
            ),
            invalid_authcode => panic!(
                "\"authcode\" is not a `String` or `Number`; actual value: {:?}",
                invalid_authcode
            ),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthSuccessBody {
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

#[derive(Deserialize, Debug, Clone)]
pub struct AuthFailureBody {
    pub authcode: u32,
    pub message: String,
    pub inactive: bool,
    #[serde(rename = "verificationRequired")]
    pub verification_required: bool,
}

/// The various ways authentication can fail
#[derive(Error, Debug, Clone)]
pub enum AuthErrorKind {
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

#[derive(Error, Debug, Clone)]
pub struct AuthError {
    #[source]
    pub kind: AuthErrorKind,
    pub body: AuthFailureBody,
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Authentication failed: {}", self.kind)
    }
}

use serde::{Deserialize, Serialize};

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

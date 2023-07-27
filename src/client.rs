use crate::{
    config::Configuration,
    error::{self, Error},
    middleware::ErrorForStatus,
    totp,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use surf::Client as SurfClient;

#[derive(Debug, Deserialize, Serialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub totp_secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TotpBody {
    totp_code: String,
    method: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TotpResponse {
    authentication_session: String,
    push_subscription_id: String,
    customer_id: String,
    registration_complete: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionUser {
    pub security_token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TwoFactorLogin {
    pub transaction_id: String,
    pub method: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCredentialsResponse {
    pub two_factor_login: TwoFactorLogin,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCredentialsRequest {
    pub username: String,
    pub password: String,
}

pub struct Client {
    pub(crate) http_client: SurfClient,
    pub(crate) config: Configuration,
}

impl Client {
    pub async fn authenticate(credentials: &Credentials) -> Result<Client, error::Error> {
        // Create default config.
        let config = Configuration::default();

        // Send initial auth request with username and password.
        let user_credentials_response = surf::post(config.urls.authenticate.as_str())
            .middleware(ErrorForStatus)
            .body_json(&UserCredentialsRequest {
                username: credentials.username.clone(),
                password: credentials.password.clone()
            })?
            .recv_json::<UserCredentialsResponse>()
            .await?;

        // Generate a totp code.
        let totp_code = totp::generate_totp(&credentials.totp_secret);

        // Send totp code and transaction id as cookie.
        let mut totp_response = surf::post(config.urls.totp.as_str())
            .middleware(ErrorForStatus)
            .header(
                "cookie",
                format!(
                    "AZAMFATRANSACTION={}",
                    user_credentials_response.two_factor_login.transaction_id
                ),
            )
            .body_json(&TotpBody {
                method: String::from("TOTP"),
                totp_code: totp_code.to_string(),
            })?
            .await?;

        // Parse the response body.
        let totp_data = totp_response.body_json::<TotpResponse>().await?;

        let authentication_token = totp_data.authentication_session.clone();

        // Extract security token from headers.
        let security_token = totp_response
            .header("x-securitytoken")
            .ok_or(Error::SecurityTokenError(String::from(
                "Header 'x-securitytoken was missing in totp request'.",
            )))?
            .as_str();

        // Create an http client.
        let http_client =
            Self::create_http_client(String::from(security_token), authentication_token.clone())?;

        // Create and return Client.
        let client = Client {
            http_client,
            config: config.clone(),
        };
        return Ok(client);
    }

    fn create_http_client(
        security_token: String,
        authentication_token: String,
    ) -> Result<SurfClient, Error> {
        let client: SurfClient = surf::Config::new()
            .set_timeout(Some(Duration::from_secs(5)))
            .add_header("X-Securitytoken", security_token)?
            .add_header("X-AuthenticationSession", authentication_token)?
            .try_into()
            .map_err(|_| {
                Error::CreateHTTPClientError(String::from("Could not create http client"))
            })?;
        let client = client.with(ErrorForStatus);

        return Ok(client);
    }
}

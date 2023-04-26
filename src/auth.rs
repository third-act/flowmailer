use crate::error::{self, Error, Result};
use serde::Deserialize;
use std::fmt::Display;

pub struct ClientSecret(String);

pub struct ClientId(String);

#[derive(Deserialize, Debug)]
pub struct AccessToken(String);

#[derive(Deserialize, Debug)]
struct AccessTokenResponse {
    access_token: AccessToken,
}

#[derive(Clone, Debug)]
pub struct AccountId(String);

impl ClientSecret {
    pub fn new(secret: String) -> Self {
        Self(secret)
    }

    pub fn secret(&self) -> &String {
        &self.0
    }
}

impl ClientId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn id(&self) -> &String {
        &self.0
    }
}

impl AccessToken {
    pub fn new(access_token: String) -> Self {
        Self(access_token)
    }

    pub fn token(&self) -> &str {
        &self.0
    }
}

impl Display for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl AccountId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub struct Auth {
    client_secret: ClientSecret,
    client_id: ClientId,
    account_id: AccountId,
}

impl Auth {
    pub fn new(client_secret: ClientSecret, client_id: ClientId, account_id: AccountId) -> Self {
        let auth = Self {
            client_secret,
            client_id,
            account_id,
        };
        auth
    }

    pub(crate) async fn authenticate(&self, reqw_client: &reqwest::Client) -> Result<AccessToken> {
        let reqw = reqw_client
            .post("https://login.flowmailer.net/oauth/token")
            .form(&[
                ("client_id", self.client_id.id().as_str()),
                ("client_secret", self.client_secret.secret().as_str()),
                ("grant_type", "client_credentials"),
                ("scope", "api"),
            ]);
        let response = match reqw.send().await {
            Ok(response) => response,
            Err(error) => return Err(Error::new(error::Kind::Other(error.to_string()))),
        };
        let response = match response.json::<AccessTokenResponse>().await {
            Ok(json) => json,
            Err(err) => {
                return Err(Error::new(error::Kind::Auth(format!(
                    "failed to parse response json: {}",
                    err.to_string(),
                ))))
            }
        };
        Ok(response.access_token)
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }
}

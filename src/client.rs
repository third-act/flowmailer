use crate::auth::{AccessToken, Auth};
use crate::error::Result;

pub struct Client {
    reqw_client: reqwest::Client,
    credentials: Auth,
}

impl Client {
    pub fn new(credentials: Auth) -> Self {
        let reqw_client = reqwest::Client::new();
        Self {
            reqw_client,
            credentials,
        }
    }

    pub fn credentials(&self) -> &Auth {
        &self.credentials
    }

    pub(crate) fn reqw_client(&self) -> &reqwest::Client {
        &self.reqw_client
    }

    pub(crate) async fn authenticate(&self) -> Result<AccessToken> {
        self.credentials.authenticate(&self.reqw_client).await
    }
}

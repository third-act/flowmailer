use crate::error::{self, Error};
use crate::{Client, Result};
use reqwest::{Body, Url};
use serde::Serialize;

pub struct RequestBuilder<'a> {
    inner: reqwest::RequestBuilder,
    client: &'a Client,
}

#[derive(Debug)]
pub struct Response(reqwest::Response);

impl Response {
    pub(crate) fn new(response: reqwest::Response) -> Self {
        Self(response)
    }

    pub(crate) fn into_inner(self) -> reqwest::Response {
        self.0
    }
}

impl<'a> RequestBuilder<'a> {
    const BASE_URL: &str = "https://api.flowmailer.net";

    pub fn post(client: &'a Client, path: &str, matrix_args: &[(&str, &str)]) -> Result<Self> {
        let url = Self::create_url(client, &path, matrix_args)?;
        let inner = client.reqw_client().post(url);
        Ok(Self { client, inner })
    }

    pub fn get(client: &'a Client, path: &str, matrix_args: &[(&str, &str)]) -> Result<Self> {
        let url = Self::create_url(client, path, matrix_args)?;
        let inner = client.reqw_client().get(url);
        Ok(Self { client, inner })
    }

    pub fn body(mut self, val: impl Into<Body>) -> Self {
        self.inner = self.inner.body(val);
        self
    }

    pub fn query(mut self, val: &impl Serialize) -> Self {
        self.inner = self.inner.query(val);
        self
    }

    pub fn form(mut self, val: &impl Serialize) -> Self {
        self.inner = self.inner.form(val);
        self
    }

    pub async fn execute(self) -> Result<Response> {
        let auth = self.client.authenticate();
        let request = self
            .inner
            .bearer_auth(auth.await?)
            .header(
                "Accept",
                "application/vnd.flowmailer.v1.12+json;charset=UTF-8",
            )
            .header("Host", "api.flowmailer.net")
            .header(
                "Content-Type",
                "application/vnd.flowmailer.v1.12+json;charset=UTF-8",
            )
            .build()?;
        let response = self.client.reqw_client().execute(request).await;
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(Response::new(response))
                } else {
                    Err(Error::new(error::Kind::BadRequest(
                        response.text().await.unwrap_or("no text body".to_string()),
                    )))
                }
            }
            Err(error)
                if error.is_status()
                    && error.status().expect("expected status error")
                        == reqwest::StatusCode::UNAUTHORIZED =>
            {
                Err(Error::new(error::Kind::AuthExpiredToken))
            }
            Err(error) => Err(Error::new(error::Kind::Other(error.to_string()))),
        }
    }

    fn create_url(
        client: &Client,
        path: &str,
        matrix_args: &[(&str, &str)],
    ) -> Result<reqwest::Url> {
        let mut url = match Url::parse(Self::BASE_URL) {
            Ok(base) => base,
            Err(err) => return Err(Error::new(error::Kind::Parse(err.to_string()))),
        };
        url.set_path(&format!(
            "{}/{}{}",
            client.credentials().account_id().as_str(),
            path,
            matrix_args
                .iter()
                .map(|(key, val)| format!(";{key}={val}"))
                .collect::<String>()
        ));
        Ok(url)
    }
}

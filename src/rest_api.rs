use crate::client::Client;
use crate::error::Result;
use crate::request::{self, Response};
use std::fmt::Debug;

pub struct Id(u64);

impl Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Id").field("id", &self.0).finish()
    }
}

pub mod message {
    use super::*;
    use crate::resources::SubmitMessage;

    const PATH: &str = "messages";

    fn subpath(path: &str) -> String {
        format!("{PATH}/{path}")
    }

    /// Send an email or sms message.
    pub async fn submit(client: &Client, msg: SubmitMessage) -> Result<Response> {
        request::RequestBuilder::post(client, &subpath("submit"), &[])?
            .body(serde_json::to_string(&msg)?)
            .execute()
            .await
    }
}

pub mod flows {
    use super::*;
    use crate::resources::Flow;

    pub async fn create(client: &Client, flow: Flow) -> Result<Response> {
        request::RequestBuilder::post(client, "flows", &[])?
            .body(serde_json::to_string(&flow)?)
            .execute()
            .await
    }
}

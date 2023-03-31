use serde::Serialize;

use crate::error::{self, Error, Result};
use crate::resources::{Attachment, SubmitMessage};
use crate::{rest_api, Client};

pub struct MailAddress(String);

pub struct MailBuilder {
    sender: MailAddress,
    recipient: MailAddress,
    text: Option<String>,
    subject: String,
    html: Option<String>,
    flow_selector: Option<String>,
    mail_data: Option<serde_json::Value>,
}

impl MailAddress {
    pub fn new(address: impl Into<String>) -> Self {
        Self(address.into())
    }
}

impl MailBuilder {
    fn new(sender: MailAddress, recipient: MailAddress) -> Self {
        Self {
            sender,
            recipient,
            subject: String::new(),
            text: None,
            html: None,
            flow_selector: None,
            mail_data: None,
        }
    }

    pub fn new_text(sender: MailAddress, recipient: MailAddress, text: impl Into<String>) -> Self {
        let mut new = Self::new(sender, recipient);
        new.text = Some(text.into());
        new
    }

    pub fn new_hmtl(sender: MailAddress, recipient: MailAddress, html: impl Into<String>) -> Self {
        let mut new = Self::new(sender, recipient);
        new.html = Some(html.into());
        new
    }

    pub fn new_template(
        sender: MailAddress,
        recipient: MailAddress,
        flow_selector: impl Into<String>,
    ) -> Self {
        let mut new = Self::new(sender, recipient);
        new.flow_selector = Some(flow_selector.into());
        new
    }

    pub fn set_data(mut self, data: impl Serialize) -> Result<Self> {
        self.mail_data = Some(serde_json::to_value(data)?);
        Ok(self)
    }

    pub fn set_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = subject.into();
        self
    }

    pub async fn send(self, client: &Client) -> Result<()> {
        let response = rest_api::message::submit(
            client,
            SubmitMessage {
                data: self.mail_data.unwrap_or(serde_json::json!({})),
                header_from_address: self.sender.0.clone(),
                header_from_name: "".to_string(),
                header_to_name: "".to_string(),
                headers: Box::new([]),
                message_type: crate::ty::MessageType::Email,
                recipient_address: self.recipient.0,
                sender_address: self.sender.0,
                subject: self.subject,
                text: self.text,
                attachments: None,
                delivery_notification_type: None,
                flow_selector: self.flow_selector,
                header_to_address: None,
                html: self.html,
                mimedata: None,
                schedule_at: None,
                tags: None,
            },
        )
        .await?
        .into_inner();
        if let Some(location) = response.headers().get("location") {
            if let Ok(_content) = location.to_str() {
                // parse url and key ID
                Ok(())
            } else {
                Err(Error::new(error::Kind::Parse(format!(
                    "failed to interpret header content as string for '{location:?}'"
                ))))
            }
        } else {
            Err(Error::new(error::Kind::MissingHeader {
                header: "location".to_string(),
                msg: "need location header in order to extract ID".to_string(),
            }))
        }
    }
}

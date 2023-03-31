use serde::{Deserialize, Serialize};

pub type Int = u64;

pub type Byte = u8;

pub type Bool = bool;

pub type String = std::string::String;

#[derive(Serialize, Deserialize)]
pub struct Base64(String);

impl Base64 {
    pub fn new(number: impl Into<String>) -> Self {
        Self(number.into())
    }
}

pub type ItemsRange = core::ops::Range<Int>;

#[derive(Serialize, Deserialize)]
pub struct RefRange {}

#[derive(Serialize, Deserialize)]
pub struct ContentRange {}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Date {
    year: u8,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl Date {
    pub fn test() -> Self {
        Self {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DateRange {}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Empty;

/// Valid values: `CUSTOM`.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum TyCustom {
    #[serde(rename = "CUSTOM")]
    Custom,
}

/// Valid values: `EMAIL`, `SMS` and `LETTER`.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum MessageType {
    #[serde(rename = "EMAIL")]
    Email,
    #[serde(rename = "SMS")]
    SMS,
    #[serde(rename = "LETTER")]
    Letter,
}

/// Only applicable when `messageType` = `SMS`.
/// Valid values: `NONE`, `FAILURE` and `DELIVERY_AND_FAILURE`.
/// Defaults to `NONE` for SMS.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum DeliveryNotificationType {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(rename = "DELIVERY_AND_FAILURE")]
    DeliveryAndFailure,
}

pub mod flow_step {
    use super::*;

    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "analytics")]
        Analytics,
        #[serde(rename = "qamail")]
        QAmail,
        #[serde(rename = "template")]
        Template,
        #[serde(rename = "subject")]
        Subject,
        #[serde(rename = "setSender")]
        SetSender,
        #[serde(rename = "addHeader")]
        AddHeader,
        #[serde(rename = "schedule")]
        Schedule,
        #[serde(rename = "rewriteRecipient")]
        RewriteRecipient,
        #[serde(rename = "resubmitMessage")]
        ResubmitMessage,
        #[serde(rename = "mailPlusContact")]
        MailPlusContact,
        #[serde(rename = "extractdata")]
        ExtractData,
        #[serde(rename = "externalData")]
        ExternalData,
        #[serde(rename = "externalContent")]
        ExternalContent,
        #[serde(rename = "archive")]
        Archive,
        #[serde(rename = "aggregate")]
        Aggregate,
        #[serde(rename = "addAttachment")]
        AddAttachment,
    }
}

pub mod flow_step_external_data {
    use super::*;

    /// HTTP request method.
    /// Valid values: `GET`, `POST`, `PUT`, `PATCH` and `DELETE`.
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum RequestMethod {
        #[serde(rename = "GET")]
        Get,
        #[serde(rename = "POST")]
        Post,
        #[serde(rename = "PUT")]
        Put,
        #[serde(rename = "PATCH")]
        Patch,
        #[serde(rename = "DELETE")]
        Delete,
    }
}

pub mod source {
    use super::*;

    /// Source type: `API`, `SMTP`, `SMTP_RCPT`, `SMTP_DOMAIN`, `SMPP` or `FLOWMAILER`.
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "APU")]
        Api,
        #[serde(rename = "SMTP")]
        Smtp,
        #[serde(rename = "SMTP_RCPT")]
        SmtpRcpt,
        #[serde(rename = "SMTP_DOMAIN")]
        SmtpDomain,
        #[serde(rename = "SMPP")]
        Smpp,
        #[serde(rename = "FLOWMAILER")]
        Flowmailer,
    }
}

pub mod template {
    use super::*;

    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum Disposition {
        #[serde(rename = "attachment")]
        Attachment,
        #[serde(rename = "inline")]
        Inline,
        #[serde(rename = "related")]
        Related,
    }

    /// Supported mime types:
    /// - text/plain
    /// - text/html
    /// - application/vnd.flowmailer.itext+pdf
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum MimeType {
        /// text/plain
        #[serde(rename = "text/plain")]
        TextPlain,
        /// text/hmtl
        #[serde(rename = "text/html")]
        TextHtml,
        /// application/vnd.flowmailer.itext+pdf
        #[serde(rename = "application/vnd.flowmailer.itext+pdf")]
        Application,
    }

    /// The only supported template engine is `freemarker-2.3.20`.
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum TemplateEngine {
        /// freemarker-2.3.20.
        #[serde(rename = "freemarker-2.3.20")]
        Freemarker,
    }
}

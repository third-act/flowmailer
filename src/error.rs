use std::fmt::Display;
use std::string::ParseError;

#[derive(Clone, Debug)]
pub enum Kind {
    AuthExpiredToken,
    Auth(String),
    Parse(String),
    BadRequest(String),
    MissingHeader { header: String, msg: String },
    Other(String),
}

#[derive(Debug)]
pub struct Error {
    pub kind: Kind,
}

pub type Result<T> = core::result::Result<T, Error>;

impl Error {
    pub(crate) fn new(kind: Kind) -> Self {
        Self { kind }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match &self.kind {
            Kind::AuthExpiredToken => "expired auth token".to_string(),
            Kind::Auth(msg) => format!("auth: {msg}"),
            Kind::Parse(msg) => format!("json: {msg}"),
            Kind::BadRequest(msg) => format!("bad request: {msg}"),
            Kind::MissingHeader { header, msg } => format!("missing header '{header}': {msg}"),
            Kind::Other(msg) => msg.to_string(),
        })
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::new(Kind::Other(value.to_string()))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::new(Kind::Parse(value.to_string()))
    }
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Self::new(Kind::Parse(value.to_string()))
    }
}

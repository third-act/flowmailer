pub extern crate reqwest;

pub mod auth;
pub mod client;
pub mod error;
pub mod mail;
pub mod request;
pub mod resources;
pub mod rest_api;
pub mod template;
pub mod ty;

#[cfg(test)]
mod test;

pub use auth::Auth;
pub use client::Client;
pub use error::Result;
pub use mail::{MailAddress, MailBuilder};
pub use template::Template;
